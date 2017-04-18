// mod iter;
mod player;

use card_wrapper::CardWrapper;
use point::Point;

// pub use self::iter::{CardIterator, CardIteratorMut};
pub use self::player::Player;
pub use std::rc::{Rc, Weak};

pub struct GameState {
    pub player: Player,
    pub opponent: Player,
    pub dragging_card: Option<CardReference>,
}

#[derive(Debug, Clone, Copy)]
pub enum CardReference {
    PlayerHand(usize),
    PlayerField(usize),
    OpponentHand(usize),
    OpponentField(usize),
}

impl GameState {
    fn update_positions_of_list(list: &mut Vec<CardWrapper>,
                                position_y: f32,
                                screen_size: &Point) {
        pub const CARD_IN_HAND_SPACING: f32 = 100.0;

        let mut position = Point::new((screen_size.x / 2f32) -
                                   (list.len() as f32 * CARD_IN_HAND_SPACING / 2f32) -
                                   ::CARD_WIDTH / 2f32,
                                   position_y);
        
        for card in list.iter_mut() {
            card.set_position(position);
            position.x += CARD_IN_HAND_SPACING;
        }
    }

    pub fn get_card(&self, reference: &CardReference) -> Option<&CardWrapper> {
        match *reference {
            CardReference::PlayerHand(index) => self.player.hand.get(index),
            CardReference::PlayerField(index) => self.player.field.get(index),
            CardReference::OpponentHand(index) => self.opponent.hand.get(index),
            CardReference::OpponentField(index) => self.opponent.field.get(index),
        }
    }

    pub fn get_card_mut(&mut self, reference: &CardReference) -> Option<&mut CardWrapper> {
        match *reference {
            CardReference::PlayerHand(index) => self.player.hand.get_mut(index),
            CardReference::PlayerField(index) => self.player.field.get_mut(index),
            CardReference::OpponentHand(index) => self.opponent.hand.get_mut(index),
            CardReference::OpponentField(index) => self.opponent.field.get_mut(index),
        }
    }


    pub fn update_card_origins(&mut self, screen_size: &Point) {
        GameState::update_positions_of_list(&mut self.player.hand,
                                            screen_size.y - ::CARD_HEIGHT / 2f32,
                                            screen_size);
        GameState::update_positions_of_list(&mut self.player.field,
                                            (screen_size.y + ::CARD_HEIGHT) / 2f32,
                                            screen_size);
    }

    pub fn mouse_moved_to(&mut self, mouse_position: &Point) {
        if let Some(reference) = self.dragging_card {
            if let Some(ref mut cardwrapper) = self.get_card_mut(&reference){
                cardwrapper.mouse_moved(mouse_position);
            }
        }
    }

    pub fn mouse_pressed_at(&mut self, mouse_position: &Point) {
        let hand_length = self.player.hand.len();
        for (index, ref mut card) in self.player.hand.iter_mut().rev().enumerate() {
            if card.contains(&mouse_position) {
                card.drag_start(mouse_position);
                self.dragging_card = Some(CardReference::PlayerHand(hand_length - index - 1));
                return;
            }
        }
        
        let field_length = self.player.field.len();
        for (index, ref mut card) in self.player.field.iter_mut().rev().enumerate() {
            if card.contains(&mouse_position) {
                card.drag_start(mouse_position);
                self.dragging_card = Some(CardReference::PlayerField(field_length - index - 1));
                return;
            }
        }
    }

    pub fn mouse_released(&mut self) {
        if let Some(reference) = self.dragging_card.take() {
            if let Some(ref mut cardwrapper) = self.get_card_mut(&reference){
                println!("Released card {:?} ({:?}) at {:?}", cardwrapper.card.debug_text(), reference, cardwrapper.drag_position());
                cardwrapper.dragging = false;
            }
        }
    }
}
