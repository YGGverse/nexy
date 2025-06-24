use clap::Parser;

/// Default port
/// https://nex.nightfall.city/nex/info/specification.txt
const PORT: u16 = 1900;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
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
    #[arg(long)]
    pub template_access_denied: Option<String>,

    /// Absolute path to the `Internal server error` template file
    #[arg(long)]
    pub template_internal_server_error: Option<String>,

    /// Absolute path to the `Not found` template file
    #[arg(long)]
    pub template_not_found: Option<String>,

    /// Absolute path to the `Welcome` template file.
    /// Unlike `template_index`, this applies only to the `public` location
    ///
    /// **Patterns**
    /// * `{list}` - entries list for the `public` directory
    #[arg(long)]
    pub template_welcome: Option<String>,

    /// Absolute path to the `Index` template file for each directory
    ///
    /// **Patterns**
    /// * `{list}` - entries list for the current directory
    #[arg(long)]
    pub template_index: Option<String>,

    /// Optimize memory usage on reading large files or stream
    #[arg(short, long, default_value_t = 1024)]
    pub read_chunk: usize,
}
