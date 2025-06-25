mod debug;
mod log;
mod storage;
mod template;

use {debug::Debug, log::Log, storage::Storage, template::Template};

/// Shared, multi-thread features for the current server session
pub struct Session {
    pub debug: Debug,
    pub log: Log,
    pub storage: Storage,
    pub template: Template,
}

impl Session {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        Ok(Self {
            debug: Debug::init(config)?,
            log: Log::init(config)?,
            storage: Storage::init(config)?,
            template: Template::init(config)?,
        })
    }
}
