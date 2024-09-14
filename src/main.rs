mod peer;
mod file;
mod chunk;
mod strategy;

use peer::Peer;
use file::File;
use chunk::Chunk;
use strategy::{PieceSelectionStrategy, RarestFirstStrategy, RandomFirstStrategy};

fn main() {
    // Manual Test 1: Test Peer file sharing and chunk downloading
    let mut peer1 = Peer::new("peer1".to_string());
    let mut peer2 = Peer::new("peer2".to_string());

    let mut file1 = File::new("file1".to_string());
    let chunk1 = Chunk::new("chunk1".to_string(), vec![1, 2, 3], "file1".to_string());
    file1.add_chunk(chunk1);
    peer1.share_file(file1);

    let mut file2 = File::new("file2".to_string());
    let chunk2 = Chunk::new("chunk2".to_string(), vec![4, 5, 6], "file2".to_string());
    file2.add_chunk(chunk2);
    peer2.share_file(file2);

    let rarest_first_strategy = RarestFirstStrategy;
    let random_first_strategy = RandomFirstStrategy;

    let chunks1 = &peer2.shared_files[0].chunks;
    let rarest_chunk = rarest_first_strategy.select_piece(chunks1);
    match rarest_chunk {
        Some(chunk) => peer1.download_chunk(chunk.clone()),
        None => println!("No chunk selected by RarestFirstStrategy"),
    }

    let chunks2 = &peer1.shared_files[0].chunks;
    let random_chunk = random_first_strategy.select_piece(chunks2);
    match random_chunk {
        Some(chunk) => peer2.download_chunk(chunk.clone()),
        None => println!("No chunk selected by RandomFirstStrategy"),
    }

    // Print results
    println!("Peer1 downloaded chunks: {:?}", peer1.downloaded_chunks.len());
    println!("Peer2 downloaded chunks: {:?}", peer2.downloaded_chunks.len());
}
