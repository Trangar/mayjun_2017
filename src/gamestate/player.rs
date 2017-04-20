use cards::{Card, ResourceType};
use card_wrapper::CardWrapper;

/// Holds information about a player
pub struct Player {
    pub name: String,
    pub health: i32,
    pub resources: Vec<(ResourceType, u8)>,

    /// Hold the cards that were in the deck when the game started
    /// This should not change
    pub original_deck: Vec<Box<Card>>,

    /// Holds the cards that are not drawn from the deck
    pub deck: Vec<Box<Card>>,

    /// Holds the cards that are currently in the players hand
    pub hand: Vec<CardWrapper>,

    /// Holds the cards that are currently on this players side of the board
    pub field: Vec<CardWrapper>,

    /// Holds the cards that are currently in the graveyard
    pub graveyard: Vec<CardWrapper>,
}

impl Player {
    /// Create a new player with the given name, 100 health and an empty deck
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

    /// Clear the board state and reset the deck
    pub fn reset_deck(&mut self) {
        self.deck.clear();
        self.hand.clear();
        self.field.clear();
        self.graveyard.clear();

        for card in &self.original_deck {
            self.deck.push(card.clone_box());
        }
    }

    /// Draw a card from the deck and put it in the players hand
    pub fn draw_card(&mut self) {
        // TODO: Pick a random index
        let index = 0;
        let card = self.deck.remove(index);
        self.hand.push(CardWrapper::new(card));
    }

    /// Draw a card and immediately play it on the field
    pub fn draw_and_play_card(&mut self) {
        let index = 0;
        let card = self.deck.remove(index);
        self.field.push(CardWrapper::new(card));
    }
}
