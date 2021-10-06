pub mod item;
pub mod pos;
pub mod tile;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Passage {
    Passable(f32), // ticks to pass (for 2-legged human)
    Unpassable,
}
