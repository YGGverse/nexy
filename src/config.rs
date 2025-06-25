use clap::Parser;

/// Default port
/// https://nex.nightfall.city/nex/info/specification.txt
const PORT: u16 = 1900;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Absolute path to the access log file
    #[arg(short, long)]
    pub access_log: Option<String>,

    /// Bind server(s) `host:port` to listen incoming connections
    ///
    /// * use `[host]:port` notation for IPv6
    #[arg(short, long, default_values_t = vec![
        std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, PORT).to_string(),
        std::net::SocketAddrV6::new(std::net::Ipv6Addr::LOCALHOST, PORT, 0, 0).to_string()
    ])]
    pub bind: Vec<String>,

    /// Debug level
    ///
    /// * `e` - error
    /// * `i` - info
    #[arg(short, long, default_value_t = String::from("ei"))]
    pub debug: String,

    /// Absolute path to the public files directory
    #[arg(short, long)]
    pub public: String,

    /// Absolute path to the `Access denied` template file
    ///
    /// * this template file can be in binary format (e.g. image)
    #[arg(long)]
    pub template_access_denied: Option<String>,

    /// Absolute path to the `Internal server error` template file
    ///
    /// * this template file can be in binary format (e.g. image)
    #[arg(long)]
    pub template_internal_server_error: Option<String>,

    /// Absolute path to the `Not found` template file
    ///
    /// * this template file can be in binary format (e.g. image)
    #[arg(long)]
    pub template_not_found: Option<String>,

    /// Absolute path to the `Welcome` template file.
    /// Unlike `template-index`, this applies only to the `public` location
    ///
    /// * this template file expects pattern and cannot be in binary format
    ///
    /// **Patterns**
    /// * `{list}` - entries list for the `public` directory
    #[arg(long)]
    pub template_welcome: Option<String>,

    /// Absolute path to the `Index` template file for each directory
    ///
    /// * this template file expects pattern and cannot be in binary format
    ///
    /// **Patterns**
    /// * `{list}` - entries list for the current directory
    #[arg(long)]
    pub template_index: Option<String>,

    /// Show files count in dir (as the alternative text for navigation links)
    #[arg(long, default_value_t = false)]
    pub list_dir_count: bool,

    /// Show directory accessed time
    #[arg(long, default_value_t = false)]
    pub list_dir_accessed: bool,

    /// Show directory created time
    #[arg(long, default_value_t = false)]
    pub list_dir_created: bool,

    /// Show directory modified time
    #[arg(long, default_value_t = false)]
    pub list_dir_modified: bool,

    /// Sort dirs by time accessed (name by default)
    #[arg(long, default_value_t = false)]
    pub list_dir_sort_accessed: bool,

    /// Sort dirs by time created (name by default)
    #[arg(long, default_value_t = false)]
    pub list_dir_sort_created: bool,

    /// Sort dirs by time modified (name by default)
    #[arg(long, default_value_t = false)]
    pub list_dir_sort_modified: bool,

    /// Sort dirs by count (name by default)
    #[arg(long, default_value_t = false)]
    pub list_dir_sort_count: bool,

    /// Sort directories in list DESC (ASC by default)
    #[arg(long, default_value_t = false)]
    pub list_dir_reverse: bool,

    /// Show file size in list (as the alternative text for navigation links)
    #[arg(long, default_value_t = false)]
    pub list_file_size: bool,

    /// Show file accessed time
    #[arg(long, default_value_t = false)]
    pub list_file_accessed: bool,

    /// Show file created time
    #[arg(long, default_value_t = false)]
    pub list_file_created: bool,

    /// Show file modified time
    #[arg(long, default_value_t = false)]
    pub list_file_modified: bool,

    /// Sort files by time accessed (name by default)
    #[arg(long, default_value_t = false)]
    pub list_file_sort_accessed: bool,

    /// Sort files by time created (name by default)
    #[arg(long, default_value_t = false)]
    pub list_file_sort_created: bool,

    /// Sort files by time modified (name by default)
    #[arg(long, default_value_t = false)]
    pub list_file_sort_modified: bool,

    /// Sort files by size (name by default)
    #[arg(long, default_value_t = false)]
    pub list_file_sort_size: bool,

    /// Sort files in list DESC (ASC by default)
    #[arg(long, default_value_t = false)]
    pub list_file_reverse: bool,

    /// Time format for listing items
    ///
    /// * use escape notation for `%` e.g. `"%%Y-%%m-%%d %%H:%%M:%%S"`
    #[arg(long, default_value_t = String::from("%Y-%m-%d %H:%M:%S"))]
    pub list_time_format: String,

    /// Optimize memory usage on reading large files or stream
    #[arg(short, long, default_value_t = 1024)]
    pub read_chunk: usize,
}
