mod access_log;
mod debug;
mod event;
mod storage;
mod template;

use {access_log::AccessLog, debug::Debug, event::Event, storage::Storage, template::Template};

/// Shared, multi-thread features for the current server session
pub struct Session {
    pub access_log: AccessLog,
    pub debug: Debug,
    pub event: Event,
    pub storage: Storage,
    pub template: Template,
}

impl Session {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        let template = Template::init(config)?;
        Ok(Self {
            access_log: AccessLog::init(config)?,
            debug: Debug::init(config)?,
            event: Event::init(
                // do not init `Connection` event if its features not in use
                template.welcome.contains("{hosts}") || template.welcome.contains("{hits}"),
            )?,
            storage: Storage::init(config)?,
            template,
        })
    }
}
