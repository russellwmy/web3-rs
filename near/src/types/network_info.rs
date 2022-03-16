use super::{Peer, Producer};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub active_peers: Vec<Peer>,
    pub num_active_peers: u64,
    pub peer_max_count: u64,
    pub sent_bytes_per_sec: u64,
    pub received_bytes_per_sec: u64,
    pub known_producers: Vec<Producer>,
}
