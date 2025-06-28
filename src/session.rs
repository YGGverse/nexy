mod access_log;
mod debug;
mod public;
mod request;
mod template;

use {access_log::AccessLog, debug::Debug, public::Public, request::Request, template::Template};

/// Shared, multi-thread features for the current server session
pub struct Session {
    pub access_log: AccessLog,
    pub debug: Debug,
    pub public: Public,
    pub request: Request,
    pub template: Template,
}

impl Session {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        let template = Template::init(config)?;
        Ok(Self {
            access_log: AccessLog::init(config)?,
            debug: Debug::init(config)?,
            public: Public::init(config)?,
            request: Request::init(
                // do not init `Connection` event if its features not in use
                template.welcome.contains("{hosts}") || template.welcome.contains("{hits}"),
            ),
            template,
        })
    }
}
