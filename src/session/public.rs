mod list_config;

use crate::response::Response;
use anyhow::{Result, bail};
use list_config::ListConfig;
use std::{
    fs::{self, Metadata},
    io::Read,
    path::PathBuf,
    str::FromStr,
};

/// In-session disk storage API
pub struct Public {
    /// Listing options
    list_config: ListConfig,
    /// Root path to storage, used also for the access validation
    public_dir: PathBuf,
    /// Streaming buffer options
    read_chunk: usize,
    /// Show hidden entries (in the directory listing)
    ///
    /// * important: this option does not prevent access to hidden files!
    show_hidden: bool,
}

impl Public {
    pub fn init(config: &crate::config::Config) -> Result<Self> {
        let public_dir = PathBuf::from_str(&config.public)?.canonicalize()?;
        let t = fs::metadata(&public_dir)?;
        if !t.is_dir() {
            bail!("Storage destination is not directory!");
        }
        if t.is_symlink() {
            bail!("Symlinks yet not supported!");
        }
        Ok(Self {
            list_config: ListConfig::init(config)?,
            public_dir,
            read_chunk: config.read_chunk,
            show_hidden: config.show_hidden,
        })
    }

    pub fn request(&self, query: &str, mut callback: impl FnMut(Response) -> bool) -> bool {
        let p = {
            // access restriction zone, change carefully!
            let mut p = PathBuf::from(&self.public_dir);
            p.push(query.trim_matches('/'));
            match p.canonicalize() {
                Ok(c) => {
                    if !c.starts_with(&self.public_dir) {
                        return callback(Response::AccessDenied { query });
                    }
                    c
                }
                Err(_) => return callback(Response::NotFound { query }),
            }
        };
        match fs::metadata(&p) {
            Ok(t) => match (t.is_dir(), t.is_file()) {
                (true, _) => callback(match self.list(&p) {
                    Ok(data) => Response::Directory {
                        query,
                        data,
                        is_root: p == self.public_dir,
                    },
                    Err(e) => Response::InternalServerError {
                        query: Some(query),
                        error: e.to_string(),
                    },
                }),
                (_, true) => match fs::File::open(p) {
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
                                    query: Some(query),
                                    error: format!("failed to read response chunk: `{e}`"),
                                });
                            }
                        }
                    },
                    Err(e) => callback(Response::InternalServerError {
                        query: Some(query),
                        error: format!("failed to read response: `{e}`"),
                    }),
                },
                _ => panic!(), // unexpected
            },
            Err(e) => callback(Response::InternalServerError {
                query: Some(query),
                error: format!("failed to read storage: `{e}`"),
            }),
        }
    }

    /// Build entries list for given `path`,
    /// sort ASC, by directories first.
    ///
    /// * make sure the `path` is allowed before call this method!
    fn list(&self, path: &PathBuf) -> Result<String> {
        use std::os::unix::fs::MetadataExt; // @TODO
        use urlencoding::encode;
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
            name: String,
        }
        /// Formatted file entry
        struct File {
            meta: Metadata,
            name: String,
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
            let meta = fs::metadata(e.path())?;
            match (meta.is_dir(), meta.is_file()) {
                (true, _) => dirs.push(Dir {
                    meta,
                    name,
                    count: fs::read_dir(e.path()).map_or(0, |i| i.count()),
                }),
                (_, true) => files.push(File { meta, name }),
                _ => continue, // @TODO symlinks support?
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
                a.name.cmp(&b.name)
            }
        });
        if dc.is_reverse {
            dirs.reverse()
        }
        for dir in dirs {
            r.push({
                let mut l = format!(
                    "=> {}/",
                    self.list_config
                        .list_url_encode
                        .as_ref()
                        .and_then(|r| if r.is_match(&dir.name) {
                            Some(encode(&dir.name).to_string())
                        } else {
                            None
                        })
                        .unwrap_or(dir.name)
                ); // link
                let mut a = Vec::new(); // alt
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
                a.name.cmp(&b.name)
            }
        });
        if fc.is_reverse {
            files.reverse()
        }
        for file in files {
            r.push({
                let mut l = format!(
                    "=> {}",
                    self.list_config
                        .list_url_encode
                        .as_ref()
                        .and_then(|r| if r.is_match(&file.name) {
                            Some(encode(&file.name).to_string())
                        } else {
                            None
                        })
                        .unwrap_or(file.name)
                ); // link
                let mut a = Vec::new(); // alt
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
}
