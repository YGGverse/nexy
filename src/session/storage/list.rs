pub struct Time {
    pub is_accessed: bool,
    pub is_created: bool,
    pub is_modified: bool,
}

pub struct Dir {
    pub time: Time,
    pub is_count: bool,
}
pub struct File {
    pub time: Time,
    pub is_size: bool,
}

pub struct List {
    pub dir: Dir,
    pub file: File,
    pub time_format: String,
}
