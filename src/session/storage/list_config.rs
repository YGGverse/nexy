pub struct Time {
    pub is_accessed: bool,
    pub is_created: bool,
    pub is_modified: bool,
}

pub struct DirAlt {
    pub time: Time,
    pub is_count: bool,
}

pub struct DirSort {
    pub time: Time,
    pub is_count: bool,
}

pub struct FileAlt {
    pub time: Time,
    pub is_size: bool,
}

pub struct FileSort {
    pub time: Time,
    pub is_size: bool,
}

pub struct Dir {
    pub alt: DirAlt,
    pub is_reverse: bool,
    pub sort: DirSort,
}
pub struct File {
    pub alt: FileAlt,
    pub is_reverse: bool,
    pub sort: FileSort,
}

pub struct ListConfig {
    pub dir: Dir,
    pub file: File,
    pub time_format: String,
}

impl ListConfig {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        use anyhow::bail;
        fn is_unique(args: &[bool]) -> bool {
            let mut c = 0;
            for a in args {
                if *a {
                    c += 1
                }
            }
            c <= 1
        }
        if !is_unique(&[
            config.list_dir_sort_accessed,
            config.list_dir_sort_created,
            config.list_dir_sort_modified,
            config.list_dir_sort_count,
        ]) {
            bail!("Dir sort option should be unique!")
        }
        if !is_unique(&[
            config.list_file_sort_accessed,
            config.list_file_sort_created,
            config.list_file_sort_modified,
            config.list_file_sort_size,
        ]) {
            bail!("File sort option should be unique!")
        }
        Ok(Self {
            dir: Dir {
                alt: DirAlt {
                    time: Time {
                        is_accessed: config.list_dir_accessed,
                        is_created: config.list_dir_created,
                        is_modified: config.list_dir_modified,
                    },
                    is_count: config.list_dir_count,
                },
                is_reverse: config.list_dir_reverse,
                sort: DirSort {
                    time: Time {
                        is_accessed: config.list_dir_sort_accessed,
                        is_created: config.list_dir_sort_created,
                        is_modified: config.list_dir_sort_modified,
                    },
                    is_count: config.list_dir_sort_count,
                },
            },
            file: File {
                alt: FileAlt {
                    time: Time {
                        is_accessed: config.list_file_accessed,
                        is_created: config.list_file_created,
                        is_modified: config.list_file_modified,
                    },
                    is_size: config.list_file_size,
                },
                is_reverse: config.list_file_reverse,
                sort: FileSort {
                    time: Time {
                        is_accessed: config.list_file_sort_accessed,
                        is_created: config.list_file_sort_created,
                        is_modified: config.list_file_sort_modified,
                    },
                    is_size: config.list_file_sort_size,
                },
            },
            time_format: config.list_time_format.clone(),
        })
    }
}
