mod list;

use crate::response::Response;
use anyhow::{Result, bail};
use list::{Dir, File, List, Time};
use std::{fs, io::Read, os::unix::fs::MetadataExt, path::PathBuf, str::FromStr};

/// In-session disk storage API
pub struct Storage {
    /// Listing options
    list: List,
    /// Root path to storage, used also for the access validation
    public_dir: PathBuf,
    /// Streaming buffer options
    read_chunk: usize,
}

impl Storage {
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
            list: List {
                dir: Dir {
                    time: Time {
                        is_accessed: config.list_dir_accessed,
                        is_created: config.list_dir_created,
                        is_modified: config.list_dir_modified,
                    },
                    is_count: config.list_dir_count,
                },
                file: File {
                    time: Time {
                        is_accessed: config.list_file_accessed,
                        is_created: config.list_file_created,
                        is_modified: config.list_file_modified,
                    },
                    is_size: config.list_file_size,
                },
                time_format: config.list_time_format.clone(),
            },
            public_dir,
            read_chunk: config.read_chunk,
        })
    }

    pub fn request(&self, query: &str, mut callback: impl FnMut(Response)) {
        let p = {
            // access restriction zone, change carefully!
            let mut p = PathBuf::from(&self.public_dir);
            p.push(query.trim_matches('/'));
            match p.canonicalize() {
                Ok(c) => {
                    if !c.starts_with(&self.public_dir) {
                        return callback(Response::AccessDenied(query));
                    }
                    c
                }
                Err(_) => return callback(Response::NotFound(query)),
            }
        };
        match fs::metadata(&p) {
            Ok(t) => match (t.is_dir(), t.is_file()) {
                (true, _) => callback(match self.list(&p) {
                    Ok(l) => Response::Directory(l, p == self.public_dir),
                    Err(e) => Response::InternalServerError(e.to_string()),
                }),
                (_, true) => match fs::File::open(p) {
                    Ok(mut f) => loop {
                        let mut b = vec![0; self.read_chunk];
                        match f.read(&mut b) {
                            Ok(0) => break,
                            Ok(n) => callback(Response::File(&b[..n])),
                            Err(e) => {
                                return callback(Response::InternalServerError(format!(
                                    "failed to read response chunk for `{query}`: `{e}`"
                                )));
                            }
                        }
                    },
                    Err(e) => callback(Response::InternalServerError(format!(
                        "failed to read response for query`{query}`: `{e}`"
                    ))),
                },
                _ => panic!(), // unexpected
            },
            Err(e) => callback(Response::InternalServerError(format!(
                "failed to read storage for `{query}`: `{e}`"
            ))),
        }
    }

    /// Build entries list for given `path`,
    /// sort ASC, by directories first.
    ///
    /// * make sure the `path` is allowed before call this method!
    fn list(&self, path: &PathBuf) -> Result<String> {
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

        // separate dirs from files, to show the dirs first
        const C: usize = 25; // @TODO optional
        let mut d = Vec::with_capacity(C);
        let mut f = Vec::with_capacity(C);
        for entry in fs::read_dir(path)? {
            let e = entry?;
            let m = fs::metadata(e.path())?;
            match (m.is_dir(), m.is_file()) {
                (true, _) => d.push((
                    e.file_name().to_string_lossy().to_string(),
                    m,
                    fs::read_dir(e.path()).map_or(0, |i| i.count()),
                )),
                (_, true) => f.push((e.file_name().to_string_lossy().to_string(), m)),
                _ => {} // @TODO symlinks support?
            }
        }
        // build resulting list
        let mut r = Vec::with_capacity(d.len() + f.len());
        // append top navigation (if not root)
        if &self.public_dir != path {
            r.push("=> ../".to_string())
        }
        // format dirs list
        d.sort_by(|(a, _, _), (b, _, _)| a.cmp(b));
        for (n, m, c) in d {
            r.push({
                let mut l = format!("=> {}/", encode(&n));
                let mut a = Vec::new();
                if self.list.dir.is_count {
                    a.push(c.to_string());
                }
                if self.list.dir.time.is_accessed {
                    a.push(self.t(m.atime()))
                }
                if self.list.dir.time.is_created {
                    a.push(self.t(m.ctime()))
                }
                if self.list.dir.time.is_modified {
                    a.push(self.t(m.mtime()))
                }
                // @TODO modified, accessed, created etc.
                if !a.is_empty() {
                    l.push_str(&format!(" ({})", a.join(",")));
                }
                l
            })
        }
        // format files list
        f.sort_by(|(a, _), (b, _)| a.cmp(b));
        for (n, m) in f {
            r.push({
                let mut l = format!("=> {}", encode(&n));
                let mut a = Vec::new();
                if self.list.file.is_size {
                    a.push(b(m.size()))
                }
                if self.list.file.time.is_accessed {
                    a.push(self.t(m.atime()))
                }
                if self.list.file.time.is_created {
                    a.push(self.t(m.ctime()))
                }
                if self.list.file.time.is_modified {
                    a.push(self.t(m.mtime()))
                }
                if !a.is_empty() {
                    l.push_str(&format!(" ({})", a.join(",")));
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
            .format(&self.list.time_format)
            .to_string()
    }
}
