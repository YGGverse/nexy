use anyhow::{Result, bail};

#[derive(PartialEq)]
pub enum Level {
    Error,
    Info,
}

impl Level {
    pub fn parse(value: char) -> Result<Self> {
        match value {
            'e' => Ok(Self::Error),
            'i' => Ok(Self::Info),
            _ => bail!("unsupported debug level `{value}`!"),
        }
    }
}
