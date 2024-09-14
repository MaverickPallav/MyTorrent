use crate::file::File;
use crate::chunk::Chunk;

pub struct Peer {
    pub peer_id: String,
    pub shared_files: Vec<File>,
    pub downloaded_chunks: Vec<Chunk>,
}

impl Peer {
    pub fn new(peer_id: String) -> Self {
        Peer {
            peer_id,
            shared_files: Vec::new(),
            downloaded_chunks: Vec::new(),
        }
    }

    pub fn share_file(&mut self, file: File) {
        self.shared_files.push(file);
    }

    pub fn download_chunk(&mut self, chunk: Chunk) {
        self.downloaded_chunks.push(chunk);
    }
}
