/// Internal server response types
pub enum Response<'a> {
    AccessDenied {
        path: std::path::PathBuf,
        query: &'a str,
    },
    InternalServerError {
        error: String,
        query: Option<&'a str>,
    },
    NotFound {
        error: String,
        query: &'a str,
    },
    File(&'a [u8]),
    Directory {
        data: String,
        is_root: bool,
        query: &'a str,
    },
}
