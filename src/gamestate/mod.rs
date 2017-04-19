mod iter;
mod player;
mod positioning;

use constants::{CARD_HEIGHT, CARD_IN_HAND_SPACING, CARD_ON_FIELD_SPACING};
use card_wrapper::CardWrapper;
use utils::VecUtils;
use point::Point;

pub use self::positioning::{AreaReference, CardReference};
pub use self::player::Player;

pub struct GameState {
    pub player: Player,
    pub opponent: Player,
    pub dragging_card: Option<CardReference>,
}

impl GameState {
    fn update_positions_of_list(list: &mut Vec<CardWrapper>,
                                position_y: f32,
                                spacing: f32,
                                screen_size: &Point) {

        let mut position = Point::new((screen_size.x / 2f32) -
                                      ((list.len() as f32 * spacing) - spacing) / 2f32,
                                      position_y);

        for card in list.iter_mut() {
            card.set_position(position);
            position.x += spacing;
        }
    }

    pub fn get_card(&self, reference: &CardReference) -> Option<&CardWrapper> {
        match reference.area {
            AreaReference::PlayerHand => self.player.hand.get(reference.index),
            AreaReference::PlayerField => self.player.field.get(reference.index),
            AreaReference::OpponentHand => self.opponent.hand.get(reference.index),
            AreaReference::OpponentField => self.opponent.field.get(reference.index),
        }
    }

    pub fn get_card_mut(&mut self, reference: &CardReference) -> Option<&mut CardWrapper> {
        match reference.area {
            AreaReference::PlayerHand => self.player.hand.get_mut(reference.index),
            AreaReference::PlayerField => self.player.field.get_mut(reference.index),
            AreaReference::OpponentHand => self.opponent.hand.get_mut(reference.index),
            AreaReference::OpponentField => self.opponent.field.get_mut(reference.index),
        }
    }

    pub fn take_card_at(&mut self, reference: &CardReference) -> Option<CardWrapper> {
        match reference.area {
            AreaReference::PlayerHand => self.player.hand.try_remove(reference.index),
            AreaReference::PlayerField => self.player.field.try_remove(reference.index),
            AreaReference::OpponentHand => self.opponent.hand.try_remove(reference.index),
            AreaReference::OpponentField => self.opponent.field.try_remove(reference.index),
        }
    }

    pub fn insert_card_at(&mut self, cardwrapper: CardWrapper, reference: &CardReference) -> bool {
        match reference.area {
            AreaReference::PlayerHand => {
                self.player.hand.push_or_insert(reference.index, cardwrapper)
            }
            AreaReference::PlayerField => {
                self.player.field.push_or_insert(reference.index, cardwrapper)
            }
            AreaReference::OpponentHand => {
                self.opponent.hand.push_or_insert(reference.index, cardwrapper)
            }
            AreaReference::OpponentField => {
                self.opponent.field.push_or_insert(reference.index, cardwrapper)
            }
        }
    }

    pub fn update_card_origins(&mut self, screen_size: &Point) {
        GameState::update_positions_of_list(&mut self.player.hand,
                                            screen_size.y - CARD_HEIGHT / 2f32,
                                            CARD_IN_HAND_SPACING,
                                            screen_size);
        GameState::update_positions_of_list(&mut self.player.field,
                                            (screen_size.y + CARD_HEIGHT) / 2f32,
                                            CARD_ON_FIELD_SPACING,
                                            screen_size);
    }

    pub fn mouse_moved_to(&mut self, mouse_position: &Point) {
        if let Some(reference) = self.dragging_card {
            if let Some(ref mut cardwrapper) = self.get_card_mut(&reference) {
                cardwrapper.mouse_moved(mouse_position);
            }
        }
    }

    pub fn mouse_pressed_at(&mut self, mouse_position: &Point) {
        let mut lists = [
            (&mut self.player.hand.iter_mut(), AreaReference::PlayerHand),
            (&mut self.player.field.iter_mut(), AreaReference::PlayerField)
        ];
        for &mut (ref mut list, area) in &mut lists {
            let length = list.len();
            for (index, ref mut card) in list.rev().enumerate() {
                if card.contains(mouse_position) {
                    card.drag_start(mouse_position);
                    self.dragging_card = Some(CardReference {
                                                area: area,
                                                index: length - index - 1,
                                            });
                    return;
                }
            }
        }

        // let hand_length = self.player.hand.len();
        // for (index, ref mut card) in
        //     self.player
        //         .hand
        //         .iter_mut()
        //         .rev()
        //         .enumerate() {
        //     if card.contains(mouse_position) {
        //         card.drag_start(mouse_position);
        //         self.dragging_card = Some(CardReference {
        //                                       area: AreaReference::PlayerHand,
        //                                       index: hand_length - index - 1,
        //                                   });
        //         return;
        //     }
        // }

        // let field_length = self.player.field.len();
        // for (index, ref mut card) in
        //     self.player
        //         .field
        //         .iter_mut()
        //         .rev()
        //         .enumerate() {
        //     if card.contains(mouse_position) {
        //         card.drag_start(mouse_position);
        //         self.dragging_card = Some(CardReference {
        //                                       area: AreaReference::PlayerField,
        //                                       index: field_length - index - 1,
        //                                   });
        //         return;
        //     }
        // }
    }

    fn get_card_index(cards: &[CardWrapper], mouse_x: f32) -> usize {
        for (index, card) in cards.iter().enumerate() {
            if card.position().x > mouse_x {
                return index;
            }
        }
        cards.len()
    }

    fn get_area_from_point(&self, point: &Point, screen_size: &Point) -> Option<CardReference> {
        let y_factor = point.y / screen_size.y;
        if y_factor > 0.5 && y_factor < 0.7 {
            Some(CardReference {
                     area: AreaReference::PlayerField,
                     index: GameState::get_card_index(&self.player.field, point.x),
                 })
        } else {
            println!("Y factor: {:?} ({:?} / {:?})",
                     y_factor,
                     point.y,
                     screen_size.y);
            None
        }
    }

    fn play_card_from_hand(&mut self,
                           start_position: &CardReference,
                           target_position: &CardReference) {
        if let Some(cardwrapper) = self.take_card_at(start_position) {
            if !self.insert_card_at(cardwrapper, target_position) {
                println!("Could not insert card at {:?}", target_position);
            }
        }
    }

    fn can_play_card_from_hand(&mut self) -> bool {
        self.player.field.len() < 7
        // if self.player.field.len() >= 7 {
        //     false
        // } else {
        //     true
        // }
    }

    pub fn mouse_released(&mut self, screen_size: &Point) {
        let mut position = None;
        if let Some(reference) = self.dragging_card.take() {
            if let Some(ref mut cardwrapper) = self.get_card_mut(&reference) {
                cardwrapper.dragging = false;
                position = Some(*cardwrapper.drag_position());
            }
            if self.can_play_card_from_hand() {
                if let Some(position) = position {
                    if let Some(position) = self.get_area_from_point(&position, screen_size) {
                        match (reference.area, position.area) {
                            (AreaReference::PlayerHand, AreaReference::PlayerField) => {
                                self.play_card_from_hand(&reference, &position);
                                self.update_card_origins(screen_size);
                            }
                            x => println!("Unknown action combination: {:?}", x),
                        }
                    }
                }
            }
        }
    }
}
