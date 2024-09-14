use sha2::{Sha256, Digest};
use hex;

#[derive(Clone)]
pub struct Chunk {
    pub chunk_id: String,
    pub data: Vec<u8>,
    pub file_id: String,
    pub hash: String, // Add hash field
}

impl Chunk {
    pub fn new(chunk_id: String, data: Vec<u8>, file_id: String) -> Self {
        let hash = compute_hash(&data);
        Chunk {
            chunk_id,
            data,
            file_id,
            hash,
        }
    }

    pub fn verify(&self) -> bool {
        self.hash == compute_hash(&self.data)
    }
}

fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}
