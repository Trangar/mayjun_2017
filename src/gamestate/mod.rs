mod iter;

use cards::{Card, ResourceType};
use card_wrapper::CardWrapper;
use point::Point;

pub use self::iter::{CardIterator, CardIteratorMut};
pub use std::rc::{Rc, Weak};

pub struct GameState {
    pub player: Player,
    pub opponent: Player,
}

impl GameState {
    fn update_positions_of_list(list: &mut Vec<CardWrapper>,
                                position_y: f32,
                                screen_size: &Point) {
        pub const CARD_IN_HAND_SPACING: f32 = 100.0;

        let left_card = Point::new((screen_size.x / 2f32) -
                                   (list.len() as f32 * CARD_IN_HAND_SPACING / 2f32) -
                                   ::CARD_WIDTH / 2f32,
                                   position_y);
        for i in 0..list.len() {
            list[i].set_position(left_card + (CARD_IN_HAND_SPACING * i as f32, 0.0).into());
        }
    }

    pub fn iter(&self) -> CardIterator {
        CardIterator::new(self)
    }

    pub fn iter_mut(&mut self) -> CardIteratorMut {
        CardIteratorMut::new(self)
    }

    pub fn update_card_origins(&mut self, screen_size: &Point) {
        GameState::update_positions_of_list(&mut self.player.hand,
                                            screen_size.y - ::CARD_HEIGHT / 2f32,
                                            screen_size);
        GameState::update_positions_of_list(&mut self.player.field,
                                            (screen_size.y + ::CARD_HEIGHT) / 2f32,
                                            screen_size);
    }
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
            graveyard: Vec::new(),
        }
    }
}
