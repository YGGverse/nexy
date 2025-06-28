use std::time::SystemTime;

pub struct Query {
    pub time: SystemTime,
    pub value: String,
}

impl Query {
    pub fn new(value: &str) -> Self {
        Self {
            time: SystemTime::now(),
            value: value.to_string(),
        }
    }
}
