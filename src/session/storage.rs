use crate::response::Response;
use anyhow::{Result, bail};
use std::{fs, io::Read, path::PathBuf, str::FromStr};

pub struct Storage {
    public_dir: PathBuf,
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
        const C: usize = 25; // @TODO optional
        let mut d = Vec::with_capacity(C);
        let mut f = Vec::with_capacity(C);
        for entry in fs::read_dir(path)? {
            let e = entry?;
            let t = fs::metadata(e.path())?;
            match (t.is_dir(), t.is_file()) {
                (true, _) => d.push(e.file_name()),
                (_, true) => f.push(e.file_name()),
                _ => {} // @TODO symlinks support?
            }
        }
        let mut l = Vec::with_capacity(d.len() + f.len());
        if &self.public_dir != path {
            l.push("=> ../".to_string())
        }
        d.sort();
        for dir in d {
            if let Some(s) = dir.to_str() {
                l.push(format!("=> {}/", encode(s)))
            }
        }
        f.sort();
        for file in f {
            if let Some(s) = file.to_str() {
                l.push(format!("=> {}", encode(s)))
            }
        }
        Ok(l.join("\n"))
    }
}
