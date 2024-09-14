use std::collections::HashSet;
use crate::file::File;
use crate::chunk::Chunk;
use tokio::task;

#[derive(Clone)]
pub enum PeerRole {
    Seeding,
    Leeching,
}

#[derive(Clone)]
pub struct Peer {
    pub peer_id: String,
    pub shared_files: Vec<File>,
    pub downloaded_chunks: Vec<Chunk>,
    pub available_chunks: HashSet<String>,
    pub role: PeerRole,
    pub total_chunks: usize,
    pub downloaded_chunks_count: usize,
}

impl Peer {
    pub fn new(peer_id: String, role: PeerRole) -> Self {
        Peer {
            peer_id,
            shared_files: Vec::new(),
            downloaded_chunks: Vec::new(),
            available_chunks: HashSet::new(),
            role,
            total_chunks: 0,
            downloaded_chunks_count: 0,
        }
    }

    pub fn share_file(&mut self, file: File) {
        self.shared_files.push(file);
        self.total_chunks = self.shared_files.iter().map(|f| f.chunks.len()).sum();
        for chunk in &self.shared_files.last().unwrap().chunks {
            self.available_chunks.insert(chunk.chunk_id.clone());
        }
        if let PeerRole::Leeching = self.role {
            self.role = PeerRole::Seeding; // Transition to seeding after sharing
        }
    }

    pub async fn download_chunk(&mut self, chunk: Chunk) {
        // Simulate chunk download
        task::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }).await.unwrap();

        self.downloaded_chunks.push(chunk);
        self.downloaded_chunks_count = self.downloaded_chunks.len();
    }

    pub fn has_chunk(&self, chunk_id: &str) -> bool {
        self.available_chunks.contains(chunk_id)
    }

    pub fn progress(&self) -> f64 {
        if self.total_chunks == 0 {
            0.0
        } else {
            (self.downloaded_chunks_count as f64 / self.total_chunks as f64) * 100.0
        }
    }
}
