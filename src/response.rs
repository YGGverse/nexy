/// Internal server response types
pub enum Response<'a> {
    /// Includes reference to the original request
    AccessDenied(&'a str),
    /// Includes query + server-side error description
    InternalServerError(Option<&'a str>, String),
    /// Includes reference to the original request
    NotFound(&'a str),
    /// Includes bytes array
    File(&'a [u8]),
    /// Includes query, list + is public root directory status
    Directory(&'a str, String, bool),
}
