#[derive(Clone)] // Derive Clone to enable cloning of Chunk
pub struct Chunk {
    pub chunk_id: String,
    pub data: Vec<u8>,
    pub file_id: String,
}

impl Chunk {
    pub fn new(chunk_id: String, data: Vec<u8>, file_id: String) -> Self {
        Chunk {
            chunk_id,
            data,
            file_id,
        }
    }
}
