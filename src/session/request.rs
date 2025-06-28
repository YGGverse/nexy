mod query;

use query::Query;
use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::RwLock,
};

/// Collect peer requests for stats and visitors count
pub struct Request {
    index: Option<RwLock<HashMap<IpAddr, Vec<Query>>>>,
    // prevent log file overflow by recording error events once
    is_max_peers_reported: RwLock<bool>,
    is_max_peer_queries_reported: RwLock<bool>,
}

impl Request {
    pub fn init(is_enabled: bool) -> Self {
        Self {
            index: if is_enabled {
                Some(RwLock::new(HashMap::with_capacity(100)))
            } else {
                None
            },
            is_max_peers_reported: RwLock::new(false),
            is_max_peer_queries_reported: RwLock::new(false),
        }
    }

    pub fn add(&self, peer: &SocketAddr, query: &str) {
        if let Some(ref this) = self.index {
            let mut index = this.write().unwrap();

            // Critical limits to forcefully free one memory slot(s) for the new record
            // * the query len is already limited by the read buffer (1024 bytes * LIMIT)
            // * make it optional @TODO
            const INDEX_MAX_PEERS: usize = 1000;
            const INDEX_MAX_PEER_QUERIES: usize = 1000;

            if index.len() >= INDEX_MAX_PEERS {
                let mut r = self.is_max_peers_reported.write().unwrap();
                if !*r {
                    *r = true;
                    eprintln!("Max peers index limit ({INDEX_MAX_PEERS}) reached");
                }
                let k = *index.keys().next().unwrap();
                index.remove(&k); // * there is no difference which one key to free for the HashMap
            }
            for queries in index.values_mut() {
                if queries.len() >= INDEX_MAX_PEER_QUERIES {
                    let mut r = self.is_max_peer_queries_reported.write().unwrap();
                    if !*r {
                        *r = true;
                        eprintln!(
                            "Max queries limit ({INDEX_MAX_PEER_QUERIES}) reached for `{peer}`"
                        );
                    }
                    queries.truncate(INDEX_MAX_PEER_QUERIES - 1) // free last slot for one query
                }
            }

            // Cleanup deprecated records
            // * this is expensive for each request, use schedule instead @TODO
            let d = chrono::Local::now().date_naive();
            for queries in index.values_mut() {
                queries.retain(|q| q.time.date_naive() == d)
            }
            index.retain(|_, q| !q.is_empty());

            // handle new record
            index
                .entry(peer.ip())
                .and_modify(|c| c.push(Query::new(query)))
                .or_insert(vec![Query::new(query)]);
        }
    }

    pub fn count(&self) -> usize {
        if let Some(ref i) = self.index {
            i.read().unwrap().len()
        } else {
            0
        }
    }

    pub fn total(&self, query_prefix: Option<&str>) -> usize {
        let mut t = 0;
        if let Some(ref i) = self.index {
            for queries in i.read().unwrap().values() {
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
