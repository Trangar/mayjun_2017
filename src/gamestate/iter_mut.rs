use super::{CardIteratorState, CardWrapper, GameState};

pub struct CardIteratorMut<'a> {
    pub gamestate: &'a mut GameState,
    pub state: CardIteratorState,
    pub index: usize,
}

impl<'a> Iterator for CardIteratorMut<'a> {
    type Item = &'a mut CardWrapper;
    fn next(&mut self) -> Option<Self::Item> {
        if let CardIteratorState::PlayerHand = self.state {
            if self.index <
               self.gamestate
                   .player
                   .hand
                   .len() {
                let val = (&mut self.gamestate.player.hand[self.index]) as *mut CardWrapper;
                self.index += 1;
                return Some(unsafe { &mut *val });
            }
            self.state = CardIteratorState::PlayerField;
            self.index = 0;
        }
        if let CardIteratorState::PlayerField = self.state {
            if self.index <
               self.gamestate
                   .player
                   .field
                   .len() {
                let val = (&mut self.gamestate.player.field[self.index]) as *mut CardWrapper;
                self.index += 1;
                return Some(unsafe { &mut *val });
            }
            self.state = CardIteratorState::OpponentHand;
            self.index = 0;
        }
        if let CardIteratorState::OpponentHand = self.state {
            if self.index <
               self.gamestate
                   .opponent
                   .hand
                   .len() {
                let val = (&mut self.gamestate.opponent.hand[self.index]) as *mut CardWrapper;
                self.index += 1;
                return Some(unsafe { &mut *val });
            }
            self.state = CardIteratorState::OpponentField;
            self.index = 0;
        }
        if let CardIteratorState::OpponentField = self.state {
            if self.index <
               self.gamestate
                   .opponent
                   .field
                   .len() {
                let val = (&mut self.gamestate.opponent.field[self.index]) as *mut CardWrapper;
                self.index += 1;
                return Some(unsafe { &mut *val });
            }
            self.state = CardIteratorState::Done;
        }
        None
    }
}
