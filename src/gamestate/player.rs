use cards::{Card, ResourceType};
use card_wrapper::CardWrapper;

pub struct Player {
    pub name: String,
    pub health: i32,
    pub resources: Vec<(ResourceType, u8)>,

    pub original_deck: Vec<Box<Card>>,
    pub deck: Vec<Box<Card>>,
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
            graveyard: Vec::new(),
        }
    }

    pub fn reset_deck(&mut self) {
        self.deck.clear();
        self.hand.clear();
        self.field.clear();
        self.graveyard.clear();

        for card in &self.original_deck {
            self.deck.push(card.clone_box());
        }
    }

    pub fn draw_card(&mut self) {
        let index = 0;
        let card = self.deck.remove(index);
        self.hand.push(CardWrapper::new(card));
    }
}
