pub struct Time {
    pub is_accessed: bool,
    pub is_created: bool,
    pub is_modified: bool,
}

pub struct Dir {
    pub is_count: bool,
    pub is_reverse: bool,
    pub time: Time,
}
pub struct File {
    pub is_reverse: bool,
    pub is_size: bool,
    pub time: Time,
}

pub struct ListConfig {
    pub dir: Dir,
    pub file: File,
    pub time_format: String,
}

impl ListConfig {
    pub fn init(config: &crate::config::Config) -> Self {
        Self {
            dir: Dir {
                time: Time {
                    is_accessed: config.list_dir_accessed,
                    is_created: config.list_dir_created,
                    is_modified: config.list_dir_modified,
                },
                is_count: config.list_dir_count,
                is_reverse: config.list_dir_reverse,
            },
            file: File {
                time: Time {
                    is_accessed: config.list_file_accessed,
                    is_created: config.list_file_created,
                    is_modified: config.list_file_modified,
                },
                is_size: config.list_file_size,
                is_reverse: config.list_file_reverse,
            },
            time_format: config.list_time_format.clone(),
        }
    }
}
