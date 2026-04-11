mod list_config;

use crate::response::Response;
use anyhow::{Result, bail};
use list_config::ListConfig;
use std::{
    collections::HashMap,
    fs::{self, Metadata},
    io::Read,
    path::{Path, PathBuf},
    str::FromStr,
    sync::{
        RwLock,
        atomic::{AtomicU64, Ordering},
    },
    time::{SystemTime, UNIX_EPOCH},
};
use walkdir::WalkDir;

/// In-session disk storage API
pub struct Public {
    /// Listing options
    list_config: ListConfig,
    /// Root path to storage
    public_dir: PathBuf,
    /// Streaming buffer options
    read_chunk: usize,
    /// Show hidden entries (in the directory listing)
    ///
    /// * important: this option does not prevent access to hidden files!
    show_hidden: bool,
    // This server supports auto-slug aliasing
    // it allows to simply present UTF-8 filenames in the directory index, according to the Nex protocol specification;
    // as the slug conversion is unrecoverable, rebuild hash map for all entries in the public dir.
    // * this method implements traversal access restriction, change carefully!
    // * it could be optional @TODO
    index: RwLock<HashMap<String, PathBuf>>,
    index_time: AtomicU64,
    index_update: u64,
}

impl Public {
    pub fn init(config: &crate::config::Config) -> Result<Self> {
        let public_dir = PathBuf::from_str(&config.public)?.canonicalize()?;
        let p = fs::metadata(&public_dir)?;
        if !p.is_dir() {
            bail!(
                "`public` path `{}` is not directory!",
                public_dir.to_string_lossy()
            )
        }
        if p.is_symlink() {
            bail!(
                "Symlink is not allowed for `public` path `{}`!",
                public_dir.to_string_lossy()
            )
        }
        Ok(Self {
            index_time: 0.into(),
            index_update: config.list_index_update,
            index: RwLock::new(HashMap::new()),
            list_config: ListConfig::init(config)?,
            public_dir,
            read_chunk: config.read_chunk,
            show_hidden: config.show_hidden,
        })
    }

    pub fn request(&self, query: &str, mut callback: impl FnMut(Response) -> bool) -> bool {
        if self.wants_reindex() {
            self.reindex();
        }
        let path = {
            match self.index.read().unwrap().get(query.trim_matches('/')) {
                Some(path) => path.clone(),
                None => {
                    return callback(Response::NotFound {
                        message: format!("Request `{query}` not found in map"),
                        path: None,
                        query,
                    });
                }
            }
        };
        match fs::metadata(&path) {
            Ok(t) => match (t.is_dir(), t.is_file()) {
                (true, _) => callback(match self.list(&path) {
                    Ok(data) => Response::Directory {
                        data,
                        is_root: path == self.public_dir,
                    },
                    Err(e) => Response::InternalServerError {
                        message: e.to_string(),
                        path: Some(path),
                        query: Some(query),
                    },
                }),
                (_, true) => match fs::File::open(&path) {
                    Ok(mut f) => loop {
                        let mut b = vec![0; self.read_chunk];
                        match f.read(&mut b) {
                            Ok(0) => return true,
                            Ok(n) => {
                                if !callback(Response::File(&b[..n])) {
                                    return false; // break reader on callback failure
                                }
                            }
                            Err(e) => {
                                return callback(Response::InternalServerError {
                                    message: format!("failed to read response chunk: `{e}`"),
                                    path: Some(path),
                                    query: Some(query),
                                });
                            }
                        }
                    },
                    Err(e) => callback(Response::InternalServerError {
                        message: format!("failed to read response: `{e}`"),
                        path: Some(path),
                        query: Some(query),
                    }),
                },
                _ => unreachable!(), // unexpected
            },
            Err(e) => callback(Response::InternalServerError {
                message: format!("failed to read storage: `{e}`"),
                path: Some(path),
                query: Some(query),
            }),
        }
    }

    /// Build entries list for given `path`,
    /// sort ASC, by directories first.
    ///
    /// * make sure the `path` is allowed before call this method!
    fn list(&self, path: &PathBuf) -> Result<String> {
        use std::os::unix::fs::MetadataExt; // @TODO
        /// Format bytes
        fn b(v: u64) -> String {
            const KB: f32 = 1024.0;
            const MB: f32 = KB * KB;
            const GB: f32 = MB * KB;
            let f = v as f32;
            if f < KB {
                format!("{v} {}", "B")
            } else if f < MB {
                format!("{:.2} KB", f / KB)
            } else if f < GB {
                format!("{:.2} MB", f / MB)
            } else {
                format!("{:.2} GB", f / GB)
            }
        }
        /// Formatted directory entry
        struct Dir {
            /// Items quantity in this directory
            count: usize,
            meta: Metadata,
            path: PathBuf,
        }
        /// Formatted file entry
        struct File {
            meta: Metadata,
            path: PathBuf,
        }
        // separate dirs from files, to show the dirs first
        const C: usize = 25; // @TODO optional
        let mut dirs = Vec::with_capacity(C);
        let mut files = Vec::with_capacity(C);
        for entry in fs::read_dir(path)? {
            let e = entry?;
            let name = e.file_name().to_string_lossy().to_string();
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }
            let path = e.path();
            let meta = fs::metadata(&path)?;
            if meta.is_dir() {
                dirs.push(Dir {
                    count: fs::read_dir(&path).map_or(0, |i| {
                        i.filter_map(Result::ok)
                            .filter(|e| {
                                self.show_hidden
                                    || !e.file_name().to_string_lossy().starts_with('.')
                            })
                            .count()
                    }),
                    meta,
                    path,
                })
            } else {
                files.push(File { meta, path })
            }
        }
        // build resulting list
        let mut r = Vec::with_capacity(dirs.len() + files.len());
        // append top navigation (if not root)
        if &self.public_dir != path {
            r.push("=> ../".to_string())
        }
        // format dirs list
        let dc = &self.list_config.dir; // just short alias
        dirs.sort_by(|a, b| {
            if dc.sort.time.is_accessed {
                a.meta.atime().cmp(&b.meta.atime())
            } else if dc.sort.time.is_created {
                a.meta.ctime().cmp(&b.meta.ctime())
            } else if dc.sort.time.is_modified {
                a.meta.mtime().cmp(&b.meta.mtime())
            } else if dc.sort.is_count {
                a.meta.size().cmp(&b.meta.size())
            } else {
                a.path.cmp(&b.path)
            }
        });
        if dc.is_reverse {
            dirs.reverse()
        }
        for dir in dirs {
            r.push({
                let n = file_name(&dir.path);
                let s = slug(&dir.path);
                let mut l = format!("=> {}/", slug(&dir.path)); // link
                let mut a = Vec::with_capacity(5); // alt
                if s != n {
                    a.push(n)
                }
                if dc.alt.time.is_accessed {
                    a.push(self.t(dir.meta.atime()))
                }
                if dc.alt.time.is_created {
                    a.push(self.t(dir.meta.ctime()))
                }
                if dc.alt.time.is_modified {
                    a.push(self.t(dir.meta.mtime()))
                }
                if dc.alt.is_count {
                    a.push(dir.count.to_string());
                }
                // @TODO modified, accessed, created etc.
                if !a.is_empty() {
                    l.push(' ');
                    l.push_str(&a.join(", "));
                }
                l
            })
        }
        // format files list
        let fc = &self.list_config.file; // just short alias
        files.sort_by(|a, b| {
            if fc.sort.time.is_accessed {
                a.meta.atime().cmp(&b.meta.atime())
            } else if fc.sort.time.is_created {
                a.meta.ctime().cmp(&b.meta.ctime())
            } else if fc.sort.time.is_modified {
                a.meta.mtime().cmp(&b.meta.mtime())
            } else if fc.sort.is_size {
                a.meta.size().cmp(&b.meta.size())
            } else {
                a.path.cmp(&b.path)
            }
        });
        if fc.is_reverse {
            files.reverse()
        }
        for file in files {
            r.push({
                let n = file_name(&file.path);
                let s = slug(&file.path);
                let mut l = format!("=> {}", slug(&file.path)); // link
                let mut a = Vec::with_capacity(5); // alt
                if s != n {
                    a.push(n)
                }
                if fc.alt.time.is_accessed {
                    a.push(self.t(file.meta.atime()))
                }
                if fc.alt.time.is_created {
                    a.push(self.t(file.meta.ctime()))
                }
                if fc.alt.time.is_modified {
                    a.push(self.t(file.meta.mtime()))
                }
                if fc.alt.is_size {
                    a.push(b(file.meta.size()))
                }
                if !l.ends_with('/') {
                    for p in &fc.append_slash {
                        if p.is_match(&l) {
                            l.push('/');
                            break;
                        }
                    }
                }
                if !a.is_empty() {
                    l.push(' ');
                    l.push_str(&a.join(", "))
                }
                l
            })
        }
        Ok(r.join("\n")) // @TODO cache option
    }

    /// Format time, according to the initiated settings
    fn t(&self, u: i64) -> String {
        chrono::DateTime::from_timestamp(u, 0)
            .unwrap()
            .format(&self.list_config.time_format)
            .to_string()
    }

    fn wants_reindex(&self) -> bool {
        let last_index_time = self.index_time.load(Ordering::Relaxed);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(last_index_time) >= self.index_update
    }

    fn reindex(&self) {
        log::debug!("reindex begin...");
        let mut index = self.index.write().unwrap();
        index.clear();
        for e in WalkDir::new(&self.public_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry = e.path();
            if let Ok(rel_path) = entry.strip_prefix(&self.public_dir) {
                let mut k = PathBuf::new();
                for part in rel_path
                    .components()
                    .filter_map(|c| PathBuf::from_str(&c.as_os_str().to_string_lossy()).ok())
                {
                    k.push(slug(&part))
                }
                assert!(
                    index
                        .insert(k.to_string_lossy().into(), entry.to_path_buf())
                        .is_none()
                )
            }
        }
        self.index_time.store(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            Ordering::Relaxed,
        );
        log::debug!("reindex completed with {} entries total.", index.len())
    }
}

fn slug(path: &Path) -> String {
    use slug::slugify;
    match path.file_stem() {
        Some(file_stem) => match path.extension() {
            Some(extension) => format!(
                "{}.{}",
                slugify(file_stem.to_string_lossy()),
                extension.to_string_lossy()
            ),
            None => slugify(file_stem.to_string_lossy()),
        },
        None => file_name(path),
    }
}

fn file_name(path: &Path) -> String {
    match path.file_name() {
        Some(file_name) => file_name.to_string_lossy().into(),
        None => "../".into(),
    }
}
