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
    pub request: Option<Request>,
    pub template: Template,
}

impl Session {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        let template = Template::init(config)?;
        Ok(Self {
            access_log: AccessLog::init(config)?,
            debug: Debug::init(config)?,
            public: Public::init(config)?,
            request: if template.welcome.contains("{hosts}")
                || template.welcome.contains("{hits}")
                || template.index.contains("{hosts}")
                || template.index.contains("{hits}")
            {
                Some(Request::new())
            } else {
                None // do not int request collector if its features not in use
            },
            template,
        })
    }
}
