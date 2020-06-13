/// Holds a reference to the different areas on the board that cardwrappers can be positioned at
/// This corresponds to a specific list of the current game state
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum AreaReference {
    PlayerHand,
    PlayerField,
    OpponentHand,
    OpponentField,
}

/// Holds a reference to a single point on the board that cardwrappers can be positioned at
/// This is a combined value of AreaReference and an index, which corresponds to a specific list and index in that list of the current game state
#[derive(Debug, Clone, Copy)]
pub struct CardReference {
    pub area: AreaReference,
    pub index: usize,
}
