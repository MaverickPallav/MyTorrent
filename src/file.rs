use crate::chunk::Chunk;

pub struct File {
    pub file_id: String,
    pub chunks: Vec<Chunk>,
}

impl File {
    pub fn new(file_id: String) -> Self {
        File {
            file_id,
            chunks: Vec::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }
}
