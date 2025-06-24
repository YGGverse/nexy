mod level;
use level::Level;

pub struct Debug(Vec<Level>);

impl Debug {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        let mut l = Vec::with_capacity(config.debug.len());
        for s in config.debug.to_lowercase().chars() {
            l.push(Level::parse(s)?);
        }
        Ok(Self(l))
    }

    pub fn error(&self, message: &str) {
        if self.0.contains(&Level::Error) {
            eprintln!("[{}] [error] {message}", now());
        }
    }

    pub fn info(&self, message: &str) {
        if self.0.contains(&Level::Info) {
            println!("[{}] [info] {message}", now());
        }
    }
}

fn now() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
