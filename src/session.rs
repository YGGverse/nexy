mod debug;
mod storage;
mod template;

use {debug::Debug, storage::Storage, template::Template};

/// Single container for the current session
pub struct Session {
    pub debug: Debug,
    pub storage: Storage,
    pub template: Template,
}

impl Session {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        Ok(Self {
            debug: Debug::init(&config.debug)?,
            storage: Storage::init(&config.public, config.read_chunk)?,
            template: Template::init(&config.template)?,
        })
    }
}
