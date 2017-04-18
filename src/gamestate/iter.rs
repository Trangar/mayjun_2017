use super::{CardWrapper, GameState};
use std::slice::{Iter, IterMut};
use std::cell::RefCell;
use std::iter::Map;
use std::rc::Rc;

pub struct CardIterator<'a> {
    // player_hand: Map<Iter<'a, Rc<RefCell<CardWrapper>>>, _>,
    // player_field: Map<Iter<'a, Rc<RefCell<CardWrapper>>>, _>,
    // opponent_hand: Map<Iter<'a, Rc<RefCell<CardWrapper>>>, _>,
    // opponent_field: Map<Iter<'a, Rc<RefCell<CardWrapper>>>, _>,
}

impl<'a> CardIterator<'a> {
    pub fn new(state: &'a GameState) -> CardIterator<'a> {
        // CardIterator {
        //     player_hand: state.player.hand.iter().map(|c| RefCell::borrow(c)),
        //     player_field: state.player.field.iter().map(|c| RefCell::borrow(c)),
        //     opponent_hand: state.opponent.hand.iter().map(|c| RefCell::borrow(c)),
        //     opponent_field: state.opponent.field.iter().map(|c| RefCell::borrow(c)),
        // }
    }
}

impl<'a> Iterator for CardIterator<'a> {
    type Item = &'a Rc<CardWrapper>;
    fn next(&mut self) -> Option<Self::Item> {
        // self.player_hand
        //     .next()
        //     .or_else(|| self.player_field.next())
        //     .or_else(|| self.opponent_hand.next())
        //     .or_else(|| self.opponent_field.next())
        None
    }
}

// pub struct CardIteratorMut<'a> {
//     player_hand: IterMut<'a, ::MutableRc<CardWrapper>>,
//     player_field: IterMut<'a, ::MutableRc<CardWrapper>>,
//     opponent_hand: IterMut<'a, ::MutableRc<CardWrapper>>,
//     opponent_field: IterMut<'a, ::MutableRc<CardWrapper>>,
// }

// impl<'a> CardIteratorMut<'a> {
//     pub fn new(state: &'a mut GameState) -> CardIteratorMut<'a> {
//         CardIteratorMut {
//             player_hand: state.player.hand.iter_mut(),
//             player_field: state.player.field.iter_mut(),
//             opponent_hand: state.opponent.hand.iter_mut(),
//             opponent_field: state.opponent.field.iter_mut(),
//         }
//     }
// }

// impl<'a> Iterator for CardIteratorMut<'a> {
//     type Item = &'a mut Rc<CardWrapper>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.player_hand
//             .next()
//             .or_else(|| self.player_field.next())
//             .or_else(|| self.opponent_hand.next())
//             .or_else(|| self.opponent_field.next())
//     }
// }
