// mod iter;
mod player;

use cards::{Card, ResourceType};
use card_wrapper::CardWrapper;
use point::Point;

// pub use self::iter::{CardIterator, CardIteratorMut};
pub use self::player::Player;
pub use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::ops::DerefMut;

pub struct GameState {
    pub player: Player,
    pub opponent: Player,
    pub dragging_card: Option<::MutableWeak<CardWrapper>>,
}

impl GameState {
    fn update_positions_of_list(list: &Vec<::MutableRc<CardWrapper>>,
                                position_y: f32,
                                screen_size: &Point) {
        pub const CARD_IN_HAND_SPACING: f32 = 100.0;

        let mut position = Point::new((screen_size.x / 2f32) -
                                   (list.len() as f32 * CARD_IN_HAND_SPACING / 2f32) -
                                   ::CARD_WIDTH / 2f32,
                                   position_y);
        
        for ref mut card in list.iter().map(|c| RefCell::borrow_mut(c)) {
            card.deref_mut().set_position(position);
            position.x += CARD_IN_HAND_SPACING;
        }
    }

    // pub fn iter(&self) -> CardIterator {
    //     CardIterator::new(self)
    // }

    // pub fn iter_mut(&mut self) -> CardIteratorMut {
    //     CardIteratorMut::new(self)
    // }

    pub fn update_card_origins(&mut self, screen_size: &Point) {
        GameState::update_positions_of_list(&mut self.player.hand,
                                            screen_size.y - ::CARD_HEIGHT / 2f32,
                                            screen_size);
        GameState::update_positions_of_list(&mut self.player.field,
                                            (screen_size.y + ::CARD_HEIGHT) / 2f32,
                                            screen_size);
    }

    pub fn mouse_moved_to(&mut self, mouse_position: &Point) {
        // if let Some(ref wrapper) = self.dragging_card {
        //     if let Some(wrapper) = Weak::upgrade(wrapper) {
        //         wrapper.mouse_moved(mouse_position);
        //     }
        // }
    }

    pub fn mouse_pressed_at(&mut self, mouse_position: &Point) {
        // for card in self.player.hand.iter_mut().rev() {
        //     if card.contains(&mouse_position) {
        //         card.drag_start(mouse_position);
        //         self.dragging_card = Some(Rc::downgrade(&card));
        //         break;
        //     }
        // }
    }
}
