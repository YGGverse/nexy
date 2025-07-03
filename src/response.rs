/// Internal server response types
pub enum Response<'a> {
    AccessDenied {
        query: &'a str,
    },
    InternalServerError {
        query: Option<&'a str>,
        error: String,
    },
    NotFound {
        query: &'a str,
        error: String,
    },
    File(&'a [u8]),
    Directory {
        query: &'a str,
        data: String,
        is_root: bool,
    },
}
