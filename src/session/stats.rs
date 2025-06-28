mod connections;
use connections::Connections;

pub struct Stats {
    pub connections: Connections,
    // another features...
}

impl Stats {
    pub fn init(is_connection_enabled: bool) -> anyhow::Result<Self> {
        Ok(Self {
            connections: Connections::init(is_connection_enabled),
        })
    }
}
