mod connection;
use connection::Connection;

pub struct Event {
    pub connection: Connection,
    // another features...
}

impl Event {
    pub fn init(is_connection_enabled: bool) -> anyhow::Result<Self> {
        Ok(Self {
            connection: Connection::init(is_connection_enabled),
        })
    }
}
