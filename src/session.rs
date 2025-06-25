mod access_log;
mod debug;
mod storage;
mod template;

use {access_log::AccessLog, debug::Debug, storage::Storage, template::Template};

/// Shared, multi-thread features for the current server session
pub struct Session {
    pub debug: Debug,
    pub access_log: AccessLog,
    pub storage: Storage,
    pub template: Template,
}

impl Session {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        Ok(Self {
            debug: Debug::init(config)?,
            access_log: AccessLog::init(config)?,
            storage: Storage::init(config)?,
            template: Template::init(config)?,
        })
    }
}
