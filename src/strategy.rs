use crate::chunk::Chunk;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait PieceSelectionStrategy {
    fn select_piece(&self, available_chunks: &[Chunk]) -> Option<Chunk>;
}

pub struct RarestFirstStrategy;

impl PieceSelectionStrategy for RarestFirstStrategy {
    fn select_piece(&self, available_chunks: &[Chunk]) -> Option<Chunk> {
        // Placeholder for rarest first piece selection strategy
        available_chunks.first().cloned() // Implement the actual logic
    }
}

pub struct RandomFirstStrategy;

impl PieceSelectionStrategy for RandomFirstStrategy {
    fn select_piece(&self, available_chunks: &[Chunk]) -> Option<Chunk> {
        let mut rng = thread_rng();
        available_chunks.choose(&mut rng).cloned()
    }
}
