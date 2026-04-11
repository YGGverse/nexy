pub struct Template {
    internal_server_error: Vec<u8>,
    not_found: Vec<u8>,
    pub index: String,
    pub welcome: String,
}

impl Template {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        use std::fs::{read, read_to_string};
        Ok(Self {
            index: match config.template_index {
                Some(ref p) => read_to_string(p)?,
                None => "{list}".into(),
            },
            internal_server_error: match config.template_internal_server_error {
                Some(ref p) => read(p)?,
                None => "Internal server error".into(),
            },
            not_found: match config.template_not_found {
                Some(ref p) => read(p)?,
                None => "Not found".into(),
            },
            welcome: match config.template_welcome {
                Some(ref p) => read_to_string(p)?,
                None => "Welcome to Nexy!\n\n{list}".into(),
            },
        })
    }

    pub fn index(&self, list: Option<&str>) -> Vec<u8> {
        self.index
            .replace("{list}", list.unwrap_or_default())
            .into()
    }

    pub fn internal_server_error(&self) -> Vec<u8> {
        self.internal_server_error.clone()
    }

    pub fn not_found(&self) -> Vec<u8> {
        self.not_found.clone()
    }

    pub fn welcome(&self, list: Option<&str>) -> Vec<u8> {
        self.welcome
            .replace("{list}", list.unwrap_or_default())
            .into()
    }
}
