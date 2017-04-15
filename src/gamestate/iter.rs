use super::{CardWrapper, GameState};

pub struct CardIterator<'a> {
    pub gamestate: &'a GameState,
    pub state: CardIteratorState,
    pub index: usize,
}

impl<'a> Iterator for CardIterator<'a> {
    type Item = &'a CardWrapper;
    fn next(&mut self) -> Option<Self::Item> {
        if let CardIteratorState::PlayerHand = self.state {
            if self.index <
               self.gamestate
                   .player
                   .hand
                   .len() {
                let ref val = self.gamestate.player.hand[self.index];
                self.index += 1;
                return Some(val);
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
                let ref val = self.gamestate.player.field[self.index];
                self.index += 1;
                return Some(val);
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
                let ref val = self.gamestate.opponent.hand[self.index];
                self.index += 1;
                return Some(val);
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
                let ref val = self.gamestate.opponent.field[self.index];
                self.index += 1;
                return Some(val);
            }
            self.state = CardIteratorState::Done;
        }
        None
    }
}

pub enum CardIteratorState {
    PlayerHand,
    PlayerField,
    OpponentHand,
    OpponentField,
    Done,
}
