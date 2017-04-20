mod positioning;
mod player;
mod iter;

pub use self::positioning::{AreaReference, CardReference};
pub use self::player::Player;

use constants::{CARD_HEIGHT, CARD_IN_HAND_SPACING, CARD_ON_FIELD_SPACING};
use card_wrapper::CardWrapper;
use cards::CardPlayEffect;
use utils::VecUtils;
use point::Point;

/// The state of the current game
/// This holds both of the players data, as well as all interactions that the player has with the board
pub struct GameState {
    // The current player
    pub player: Player,

    // The players opponent
    pub opponent: Player,

    // If the player is dragging a card, we hold a reference here
    pub dragging_card: Option<CardReference>,
}

impl GameState {
    pub fn new(player: Player, opponent: Player) -> GameState {
        GameState {
            player: player,
            opponent: opponent,
            dragging_card: None
        }
    }

    /// Update the positions of the cards in the given `list`
    /// They will be positioned in the center of the screen, at y coordinate `position_y`
    /// There will be `spacing` amount of pixels between the middle of the cards, not between the sides
    /// Finally, `screen_size` needs to be passed to calculate the position correctly
    fn update_positions_of_list(list: &mut Vec<CardWrapper>,
                                position_y: f32,
                                spacing: f32,
                                screen_size: &Point) {

        // calculate the position of the left-most card
        let mut position = Point::new((screen_size.x / 2f32) -
                                      ((list.len() as f32 * spacing) - spacing) / 2f32,
                                      position_y);

        for card in list.iter_mut() {
            card.set_position(position);
            position.x += spacing; // the next card is a simple `spacing` away
        }
    }

    /// Get a card based on the given CardReference
    /// This will be None if the given `reference.index` is out of range of the list
    pub fn get_card(&self, reference: &CardReference) -> Option<&CardWrapper> {
        match reference.area {
            AreaReference::PlayerHand => self.player.hand.get(reference.index),
            AreaReference::PlayerField => self.player.field.get(reference.index),
            AreaReference::OpponentHand => self.opponent.hand.get(reference.index),
            AreaReference::OpponentField => self.opponent.field.get(reference.index),
        }
    }

    /// Get a mutable reference to a card based on the given CardReference
    /// This will be None if the given `reference.index` is out of range of the list
    pub fn get_card_mut(&mut self, reference: &CardReference) -> Option<&mut CardWrapper> {
        match reference.area {
            AreaReference::PlayerHand => self.player.hand.get_mut(reference.index),
            AreaReference::PlayerField => self.player.field.get_mut(reference.index),
            AreaReference::OpponentHand => self.opponent.hand.get_mut(reference.index),
            AreaReference::OpponentField => self.opponent.field.get_mut(reference.index),
        }
    }

    /// Remove a card at the requested CardReference
    /// This will be None if the given `reference.index` is out of range of the list
    pub fn take_card_at(&mut self, reference: &CardReference) -> Option<CardWrapper> {
        match reference.area {
            AreaReference::PlayerHand => self.player.hand.try_remove(reference.index),
            AreaReference::PlayerField => self.player.field.try_remove(reference.index),
            AreaReference::OpponentHand => self.opponent.hand.try_remove(reference.index),
            AreaReference::OpponentField => self.opponent.field.try_remove(reference.index),
        }
    }

    /// Insert a card at the requested CardReference
    /// Cards can only be appended at `index <= list.len()`
    /// If a position is requested outside of this (e.g. reference.index = 5 while the list only has 3 items)
    /// this will fail and false will be returned
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

    /// Update the position of all cards. This should be called after a screen resize or a card position change so all cards are 
    /// rendered at the right position.
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

    /// Move the mouse to a given position.
    /// If `self.dragging_card` is not None, this will move the selected card with the mouse position
    pub fn mouse_moved_to(&mut self, mouse_position: &Point) {
        if let Some(reference) = self.dragging_card {
            if let Some(ref mut cardwrapper) = self.get_card_mut(&reference) {
                cardwrapper.mouse_moved(mouse_position);
            }
        }
    }

    /// Get the result of a card drag event based on the area that the card came from, and
    /// what the card is allowed to do based on the result of `wrapper.card.play_effects()`
    ///
    /// Returns the first valid combination, or None if no valid combination is found
    fn get_card_drag_result(wrapper: &CardWrapper, area: &AreaReference) -> Option<CardPlayEffect> {
        for effect in wrapper.card.play_effects() {
            match (effect, area) {
                (CardPlayEffect::SummonMinion, &AreaReference::PlayerHand) => return Some(CardPlayEffect::SummonMinion),
                (CardPlayEffect::SummonMinion, _) => {},
                (x, _) => return Some(x)
            }
        }
        None
    }

    /// Detects if the mouse is currently over a card
    /// If it is, and the card can be dragged, it'll make this card drag and 
    /// follow the mouse position on subsequential `mouse_moved_to` calls
    pub fn mouse_pressed_at(&mut self, mouse_position: &Point) {
        // Make a list of card lists and the position the card is at
        let mut lists = [
            (&mut self.player.hand.iter_mut(), AreaReference::PlayerHand),
            (&mut self.player.field.iter_mut(), AreaReference::PlayerField)
        ];
        for &mut (ref mut list, area) in &mut lists {
            let length = list.len();
            // The right-most card should be the one on top, so we need to iterate through the list reversely
            for (index, ref mut card) in list.rev().enumerate() {
                if card.contains(mouse_position) {
                    let position = CardReference {
                        area: area,
                        index: length - index - 1 // because we're iterating from the end, we need to correct the card index
                    };
                    match GameState::get_card_drag_result(&card, &area) {
                        None => {},
                        Some(CardPlayEffect::SummonMinion) => {
                            // if we can play this card, make it draggable and return
                            card.drag_start(mouse_position);
                            self.dragging_card = Some(position);
                            return;
                        },
                        Some(CardPlayEffect::Target(target)) => {
                            // if we can target something, show a targetting cursor
                            println!("Card play effect is target {:?}", target);
                        }
                    }
                }
            }
        }
    }

    /// Based on the given list of CardWrappers
    /// Get the index that the mouse is hovering at, based on the mouse's x position
    fn get_card_index(cards: &[CardWrapper], mouse_x: f32) -> usize {
        for (index, card) in cards.iter().enumerate() {
            if card.drag_position().x > mouse_x {
                return index;
            }
        }
        cards.len()
    }

    /// Get the play area and the card index of the point on the screen
    /// This returns a CardReference with the area, and the index that a card should be placed at
    /// Returns None if no valid area could be found
    fn get_area_from_point(&self, point: &Point, screen_size: &Point) -> Option<CardReference> {
        let y_factor = point.y / screen_size.y;
        if y_factor > 0.5 && y_factor < 0.75 {
            Some(CardReference {
                area: AreaReference::PlayerField,
                index: GameState::get_card_index(&self.player.field, point.x),
            })
        } else {
            // TODO: Implement the other areas
            // Especially the opponent field for targetting attacks with minions
            println!("Y factor: {:?} ({:?} / {:?})",
                     y_factor,
                     point.y,
                     screen_size.y);
            None
        }
    }

    /// Play a card from hand. This moves a card from the given `start_position` to the `target_position`
    /// This does not technically have to be a `_from_hand` function, as this will move the cards from anywhere to anywhere
    /// As long as the `start_position` and `target_position` are valid
    fn play_card_from_hand(&mut self,
                           start_position: &CardReference,
                           target_position: &CardReference) {
        if let Some(cardwrapper) = self.take_card_at(start_position) {
            if !self.insert_card_at(cardwrapper, target_position) {
                println!("Could not insert card at {:?}", target_position);
            }
        }
    }

    /// Determines if the player can play a card from hand
    /// This is currently just a check to see if the player has less than 7 minions in play
    fn can_play_card_from_hand(&mut self) -> bool {
        self.player.field.len() < 7
        // if self.player.field.len() >= 7 {
        //     false
        // } else {
        //     true
        // }
    }

    /// Trigger a mouse release event
    /// If we're dragging a card, this will attempt to play that card from hand
    /// Because we need to call `update_card_origins` and `get_area_from_point`, we also need the `screen_size` argument
    pub fn mouse_released(&mut self, screen_size: &Point) {
        let mut position = None;
        if let Some(reference) = self.dragging_card.take() {
            if let Some(ref mut cardwrapper) = self.get_card_mut(&reference) {
                cardwrapper.dragging = false;
                position = Some(*cardwrapper.drag_position());
            }
            if self.can_play_card_from_hand() {
                // TODO: Combine these two if statements?
                // Maybe even join it with the match statement?
                if let Some(position) = position {
                    if let Some(position) = self.get_area_from_point(&position, screen_size) {
                        match (reference.area, position.area) {
                            (AreaReference::PlayerHand, AreaReference::PlayerField) => {
                                // if we're playing a card from hand and to the field, play it and update the card positions
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
