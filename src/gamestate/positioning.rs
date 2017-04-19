
#[derive(Debug, Clone, Copy)]
pub enum AreaReference {
    PlayerHand,
    PlayerField,
    OpponentHand,
    OpponentField,
}

#[derive(Debug, Clone, Copy)]
pub struct CardReference {
    pub area: AreaReference,
    pub index: usize,
}
