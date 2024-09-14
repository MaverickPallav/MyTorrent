use std::collections::HashMap;
use crate::peer::Peer;

pub struct PeerRegistry {
    peers: HashMap<String, Peer>,
}

impl PeerRegistry {
    pub fn new() -> Self {
        PeerRegistry {
            peers: HashMap::new(),
        }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        self.peers.insert(peer.peer_id.clone(), peer);
    }

    pub fn get_peer(&self, peer_id: &str) -> Option<&Peer> {
        self.peers.get(peer_id)
    }

    pub fn list_peers(&self) -> Vec<String> {
        self.peers.keys().cloned().collect()
    }
}
