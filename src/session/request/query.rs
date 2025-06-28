use chrono::{DateTime, Local};

pub struct Query {
    pub time: DateTime<Local>,
    pub value: String,
}

impl Query {
    pub fn new(value: &str) -> Self {
        Self {
            time: Local::now(),
            value: value.to_string(),
        }
    }
}
