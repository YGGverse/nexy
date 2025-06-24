/// Internal types
pub enum Response<'a> {
    /// Includes reference to the original request
    AccessDenied(&'a str),
    /// Includes server-side error description
    InternalServerError(String),
    /// Includes reference to the original request
    NotFound(&'a str),
    /// Includes bytes array
    Success(&'a [u8]),
}
