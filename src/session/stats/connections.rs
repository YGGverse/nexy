use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::RwLock,
};

/// Count peer connections (for the current server session)
pub struct Connections(Option<RwLock<HashMap<IpAddr, usize>>>);

impl Connections {
    pub fn init(is_enabled: bool) -> Self {
        if is_enabled {
            Self(Some(RwLock::new(HashMap::with_capacity(100))))
        } else {
            Self(None)
        }
    }

    pub fn add(&self, peer: &SocketAddr) {
        if let Some(ref this) = self.0 {
            this.write()
                .unwrap()
                .entry(peer.ip())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }
    }

    pub fn count(&self) -> usize {
        if let Some(ref this) = self.0 {
            this.read().unwrap().len()
        } else {
            0
        }
    }

    pub fn total(&self) -> usize {
        if let Some(ref this) = self.0 {
            this.read().unwrap().values().sum()
        } else {
            0
        }
    }
}
