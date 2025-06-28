pub struct Template {
    access_denied: Vec<u8>,
    index: String,
    internal_server_error: Vec<u8>,
    not_found: Vec<u8>,
    pub welcome: String,
}

impl Template {
    pub fn init(config: &crate::config::Config) -> anyhow::Result<Self> {
        use std::fs::{read, read_to_string};
        Ok(Self {
            access_denied: match config.template_access_denied {
                Some(ref p) => read(p)?,
                None => "Access denied".into(),
            },
            index: match config.template_index {
                Some(ref p) => read_to_string(p)?,
                None => "{list}\n\n👁 {hosts} / {hits}".into(),
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
                None => "Welcome to Nexy!\n\n{list}\n\n👁 {hosts} / {hits}".into(),
            },
        })
    }

    pub fn access_denied(&self) -> &[u8] {
        &self.access_denied
    }

    pub fn index(&self, list: Option<&str>, hosts: Option<usize>, hits: Option<usize>) -> Vec<u8> {
        self.index
            .replace("{list}", list.unwrap_or_default())
            .replace("{hosts}", &format_count(hosts))
            .replace("{hits}", &format_count(hits))
            .into()
    }

    pub fn internal_server_error(&self) -> &[u8] {
        &self.internal_server_error
    }

    pub fn not_found(&self) -> &[u8] {
        &self.not_found
    }

    pub fn welcome(
        &self,
        list: Option<&str>,
        hosts: Option<usize>,
        hits: Option<usize>,
    ) -> Vec<u8> {
        self.welcome
            .replace("{list}", list.unwrap_or_default())
            .replace("{hosts}", &format_count(hosts))
            .replace("{hits}", &format_count(hits))
            .into()
    }
}

fn format_count(v: Option<usize>) -> String {
    v.map_or(String::new(), |c| c.to_string())
}
