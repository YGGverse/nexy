use std::path::PathBuf;

/// Internal server response types
pub enum Response<'a> {
    AccessDenied {
        canonical: PathBuf,
        path: PathBuf,
        query: &'a str,
    },
    InternalServerError {
        error: String,
        path: Option<PathBuf>,
        query: Option<&'a str>,
    },
    NotFound {
        error: String,
        path: PathBuf,
        query: &'a str,
    },
    File(&'a [u8]),
    Directory {
        data: String,
        is_root: bool,
        query: &'a str,
    },
}
