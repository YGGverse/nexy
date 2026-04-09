mod access_log;
mod public;
mod template;

use {access_log::AccessLog, public::Public, template::Template};

/// Shared, multi-thread features for the current server session
pub struct Session {
    pub access_log: Option<AccessLog>,
    pub public: Public,
    pub template: Template,
}

impl Session {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        let template = Template::init(config)?;
        Ok(Self {
            access_log: AccessLog::init(config)?,
            public: Public::init(config)?,
            template,
        })
    }
}
