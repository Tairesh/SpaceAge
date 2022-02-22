use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

pub const QUADRANT_SIZE: usize = 64;
pub const CHUNKS_IN_QUADRANT: usize = QUADRANT_SIZE * QUADRANT_SIZE;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quadrant {
    #[serde(rename = "s")]
    pub stars_count: u32,
    #[serde(rename = "c")]
    pub chunks: ArrayVec<Option<Chunk>, CHUNKS_IN_QUADRANT>,
}

impl Quadrant {
    pub fn new(stars_count: u32) -> Self {
        Self {
            stars_count,
            chunks: ArrayVec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chunk {}
