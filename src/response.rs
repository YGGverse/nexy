/// Internal types
pub enum Response<'a> {
    /// Includes reference to the original request
    AccessDenied(&'a str),
    /// Includes server-side error description
    InternalServerError(String),
    /// Includes reference to the original request
    NotFound(&'a str),
    /// Includes bytes array
    File(&'a [u8]),
    /// Includes bytes array + public root directory status
    Directory(String, bool),
}
