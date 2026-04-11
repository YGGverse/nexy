use std::path::PathBuf;

/// Internal server response types
pub enum Response<'a> {
    InternalServerError {
        message: String,
        path: Option<PathBuf>,
        query: Option<&'a str>,
    },
    NotFound {
        message: String,
        path: Option<PathBuf>,
        query: &'a str,
    },
    File(&'a [u8]),
    Directory {
        data: String,
        is_root: bool,
    },
}
