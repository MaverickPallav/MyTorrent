use clap::{Arg, Command};
use log::{info, error};
use env_logger;

mod peer;
mod file;
mod chunk;
mod strategy;
mod peer_registry; // Add this line

use peer::{Peer, PeerRole};
use file::File;
use chunk::Chunk;
use strategy::{PieceSelectionStrategy, RarestFirstStrategy, RandomFirstStrategy};
use peer_registry::PeerRegistry; // Add this line

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = Command::new("Torrent-like System")
        .version("1.0")
        .author("Your Name")
        .about("Simulates a Torrent-like file sharing system")
        .arg(Arg::new("peer_id")
            .short('p')
            .long("peer_id")
            .value_name("PEER_ID")
            .help("Sets the peer ID")
            .takes_value(true))
        .get_matches();

    let peer_id = matches.value_of("peer_id").unwrap_or("default_peer").to_string();
    info!("Starting system with Peer ID: {}", peer_id);

    let mut peer_registry = PeerRegistry::new(); // Create a PeerRegistry

    let mut peer1 = Peer::new(peer_id.clone(), PeerRole::Leeching);
    let mut peer2 = Peer::new("peer2".to_string(), PeerRole::Leeching);

    peer_registry.add_peer(peer1.clone()); // Add peers to the registry
    peer_registry.add_peer(peer2.clone());

    // Retrieve and use peers from the registry
    if let Some(peer) = peer_registry.get_peer(&peer_id) {
        info!("Retrieved peer from registry: {:?}", peer.peer_id);
    }

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

    // let chunk_id_to_check = "chunk1";
    // if peer1.has_chunk(chunk_id_to_check) {
    //     info!("Peer1 has chunk: {}", chunk_id_to_check);
    // } else {
    //     info!("Peer1 does not have chunk: {}", chunk_id_to_check);
    // }

    // let chunk_id_to_check = "chunk2";
    // if peer2.has_chunk(chunk_id_to_check) {
    //     info!("Peer2 has chunk: {}", chunk_id_to_check);
    // } else {
    //     info!("Peer2 does not have chunk: {}", chunk_id_to_check);
    // }

    if let Some(rarest_chunk) = rarest_first_strategy.select_piece(&peer2.shared_files[0].chunks) {
        peer1.download_chunk(rarest_chunk.clone()).await;
        info!("Peer1 downloaded chunk: {}", rarest_chunk.chunk_id);
    } else {
        error!("No chunk selected by RarestFirstStrategy");
    }

    if let Some(random_chunk) = random_first_strategy.select_piece(&peer1.shared_files[0].chunks) {
        peer2.download_chunk(random_chunk.clone()).await;
        info!("Peer2 downloaded chunk: {}", random_chunk.chunk_id);
    } else {
        error!("No chunk selected by RandomFirstStrategy");
    }

    // println!("Peer1 download progress: {:.2}%", peer1.progress());
    // println!("Peer2 download progress: {:.2}%", peer2.progress());

    println!("Peer1 downloaded chunks: {:?}", peer1.downloaded_chunks.len());
    println!("Peer2 downloaded chunks: {:?}", peer2.downloaded_chunks.len());

    // List all peers
    println!("All peers in registry: {:?}", peer_registry.list_peers());
}
