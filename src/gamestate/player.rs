use cards::{Card, ResourceType};
use card_wrapper::CardWrapper;
use point::Point;

pub struct Player {
    pub name: String,
    pub health: i32,
    pub resources: Vec<(ResourceType, u8)>,

    pub original_deck: Vec<::MutableRc<Card>>,
    pub deck: Vec<::MutableWeak<Card>>,
    pub hand: Vec<::MutableRc<CardWrapper>>,
    pub field: Vec<::MutableRc<CardWrapper>>,
    pub graveyard: Vec<::MutableRc<CardWrapper>>,
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
            graveyard: Vec::new(),
        }
    }
}
