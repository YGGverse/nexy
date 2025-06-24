use anyhow::{Result, bail};

pub struct Template {
    pub access_denied: String,
    pub internal_server_error: String,
    pub not_found: String,
}

impl Template {
    pub fn init(directory: &Option<String>) -> Result<Self> {
        use std::{fs::read_to_string, path::PathBuf};

        const ACCESS_DENIED: (&str, &str) = ("access_denied.txt", "Access denied");
        const INTERNAL_SERVER_ERROR: (&str, &str) =
            ("internal_server_error.txt", "Internal server error");
        const NOT_FOUND: (&str, &str) = ("not_found.txt", "Not found");

        fn path(directory: &str, file: &str) -> Result<PathBuf> {
            let mut p = PathBuf::from(directory).canonicalize()?;
            p.push(file);
            if !p.exists() {
                bail!("Template path `{}` does not exist", p.to_string_lossy())
            }
            if !p.is_file() {
                bail!("Template path `{}` is not file", p.to_string_lossy())
            }
            if p.is_symlink() {
                bail!("Symlinks yet not supported!");
            }
            Ok(p)
        }

        Ok(match directory {
            Some(d) => Self {
                access_denied: read_to_string(path(d, ACCESS_DENIED.0)?)?,
                internal_server_error: read_to_string(path(d, INTERNAL_SERVER_ERROR.0)?)?,
                not_found: read_to_string(path(d, NOT_FOUND.0)?)?,
            },
            None => Self {
                access_denied: ACCESS_DENIED.1.to_string(),
                internal_server_error: INTERNAL_SERVER_ERROR.1.to_string(),
                not_found: NOT_FOUND.1.to_string(),
            },
        })
    }
}
