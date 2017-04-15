use super::{CardWrapper, GameState};
use std::slice::{Iter, IterMut};

pub struct CardIterator<'a> {
    player_hand: Iter<'a, CardWrapper>,
    player_field: Iter<'a, CardWrapper>,
    opponent_hand: Iter<'a, CardWrapper>,
    opponent_field: Iter<'a, CardWrapper>,
}

impl<'a> CardIterator<'a> {
    pub fn new(state: &'a GameState) -> CardIterator<'a> {
        CardIterator {
            player_hand: state.player.hand.iter(),
            player_field: state.player.field.iter(),
            opponent_hand: state.opponent.hand.iter(),
            opponent_field: state.opponent.field.iter(),
        }
    }
}

impl<'a> Iterator for CardIterator<'a> {
    type Item = &'a CardWrapper;
    fn next(&mut self) -> Option<Self::Item> {
        self.player_hand
            .next()
            .or_else(|| self.player_field.next())
            .or_else(|| self.opponent_hand.next())
            .or_else(|| self.opponent_field.next())
    }
}

pub struct CardIteratorMut<'a> {
    player_hand: IterMut<'a, CardWrapper>,
    player_field: IterMut<'a, CardWrapper>,
    opponent_hand: IterMut<'a, CardWrapper>,
    opponent_field: IterMut<'a, CardWrapper>,
}

impl<'a> CardIteratorMut<'a> {
    pub fn new(state: &'a mut GameState) -> CardIteratorMut<'a> {
        CardIteratorMut {
            player_hand: state.player.hand.iter_mut(),
            player_field: state.player.field.iter_mut(),
            opponent_hand: state.opponent.hand.iter_mut(),
            opponent_field: state.opponent.field.iter_mut(),
        }
    }
}

impl<'a> Iterator for CardIteratorMut<'a> {
    type Item = &'a mut CardWrapper;
    fn next(&mut self) -> Option<Self::Item> {
        self.player_hand
            .next()
            .or_else(|| self.player_field.next())
            .or_else(|| self.opponent_hand.next())
            .or_else(|| self.opponent_field.next())
    }
}
