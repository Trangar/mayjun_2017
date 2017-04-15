use cards::{Card, CardWrapper, ResourceType};
pub use std::rc::{Rc, Weak};

pub struct GameState {
    pub player: Player,
    pub opponent: Player,
}

pub struct Player {
    pub name: String,
    pub health: i32,
    pub resources: Vec<(ResourceType, u8)>,

    pub original_deck: Vec<Rc<Card>>,
    pub deck: Vec<Weak<Card>>,
    pub hand: Vec<CardWrapper>,
    pub field: Vec<CardWrapper>,
    pub graveyard: Vec<CardWrapper>,
}

impl Player {
    pub fn new<T: ToString>(name: T) -> Player {
        Player {
            name: name.to_string(),
            health: 100,
            original_deck: Vec::new(),
            resources: Vec::new(),
            deck: Vec::new(),
            hand: Vec::new(),
            field: Vec::new(),
            graveyard: Vec::new()
        }
    }
}
