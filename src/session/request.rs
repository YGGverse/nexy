mod query;

use query::Query;
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::RwLock,
};

/// Collect peer requests to print stats and visitors count
pub struct Request(Option<RwLock<HashMap<IpAddr, Vec<Query>>>>);

impl Request {
    pub fn init(is_enabled: bool) -> Self {
        if is_enabled {
            Self(Some(RwLock::new(HashMap::with_capacity(100))))
        } else {
            Self(None)
        }
    }

    pub fn add(&self, peer: &SocketAddr, query: &str) {
        if let Some(ref this) = self.0 {
            this.write()
                .unwrap()
                .entry(peer.ip())
                .and_modify(|c| c.push(Query::new(query)))
                .or_insert(vec![Query::new(query)]);
        }
    }

    pub fn count(&self) -> usize {
        if let Some(ref this) = self.0 {
            this.read().unwrap().len()
        } else {
            0
        }
    }

    pub fn total(&self, query_prefix: Option<&str>) -> usize {
        let mut t = 0;
        if let Some(ref this) = self.0 {
            for queries in this.read().unwrap().values() {
                match query_prefix {
                    Some(p) => {
                        for q in queries {
                            if q.value.starts_with(p) {
                                t += 1
                            }
                        }
                    }
                    None => t += queries.len(),
                }
            }
        }
        t
    }
}
