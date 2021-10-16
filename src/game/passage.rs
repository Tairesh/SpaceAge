#[derive(Debug, Eq, PartialEq)]
pub enum Passage {
    Passable(u8), // ticks to pass (for 2-legged human)
    Unpassable,
}
