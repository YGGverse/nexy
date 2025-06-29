//! Standard access logs feature
//! that is compatible with analytics tools such as [GoAccess](https://goaccess.io/),
//! [GoatCounter](https://www.goatcounter.com/) or [htcount](https://github.com/yggverse/htcount)

use std::{fs::File, io::Write, net::SocketAddr, sync::RwLock};

/// Writes log as
pub struct AccessLog(RwLock<File>);

impl AccessLog {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Option<Self>> {
        Ok(match config.access_log {
            Some(ref p) => Some(Self(RwLock::new(File::create(p)?))),
            None => None,
        })
    }
    /// [CLF](https://en.wikipedia.org/wiki/Common_Log_Format)
    ///
    /// * the code value (`u8`) is relative, use 1|0 for failure / success
    pub fn clf(&self, client: &SocketAddr, query: Option<&str>, code: u8, size: usize) {
        self.0
            .write()
            .unwrap()
            .write_all(
                format!(
                    "{} {} - [{}] \"GET {}\" {code} {size}\n",
                    client.ip(),
                    client.port(),
                    chrono::Local::now().format("%d/%b/%Y:%H:%M:%S %z"),
                    query.unwrap_or_default(),
                )
                .as_bytes(),
            )
            .unwrap()
    }
}
