pub struct Template {
    access_denied: Vec<u8>,
    index: Vec<u8>,
    internal_server_error: Vec<u8>,
    not_found: Vec<u8>,
    welcome: Vec<u8>,
}

impl Template {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        use std::fs::read;
        Ok(Self {
            access_denied: match config.template_access_denied {
                Some(ref p) => read(p)?,
                None => "Access denied".into(),
            },
            index: match config.template_access_denied {
                Some(ref p) => read(p)?,
                None => "{list}".into(),
            },
            internal_server_error: match config.template_access_denied {
                Some(ref p) => read(p)?,
                None => "Internal server error".into(),
            },
            not_found: match config.template_access_denied {
                Some(ref p) => read(p)?,
                None => "Not found".into(),
            },
            welcome: match config.template_access_denied {
                Some(ref p) => read(p)?,
                None => "Welcome to Nexy!\n{list}".into(),
            },
        })
    }

    pub fn access_denied(&self) -> &[u8] {
        &self.access_denied
    }

    pub fn index(&self, list: Option<&str>) -> Vec<u8> {
        let l = list.unwrap_or_default();
        match std::str::from_utf8(&self.index) {
            Ok(s) => s.replace("{list}", l).into(),
            Err(_) => l.into(),
        }
    }

    pub fn internal_server_error(&self) -> &[u8] {
        &self.internal_server_error
    }

    pub fn not_found(&self) -> &[u8] {
        &self.not_found
    }

    pub fn welcome(&self, list: Option<&str>) -> Vec<u8> {
        let l = list.unwrap_or_default();
        match std::str::from_utf8(&self.welcome) {
            Ok(s) => s.replace("{list}", l).into(),
            Err(_) => l.into(),
        }
    }
}
