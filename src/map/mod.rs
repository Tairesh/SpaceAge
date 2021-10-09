pub mod pos;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Passage {
    Passable(f32), // ticks to pass (for 2-legged human)
    Unpassable,
}
