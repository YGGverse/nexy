/// Internal server response types
pub enum Response {
    InternalServerError { message: String },
    NotFound { message: String },
    File(Vec<u8>),
    Directory { data: String, is_root: bool },
}
