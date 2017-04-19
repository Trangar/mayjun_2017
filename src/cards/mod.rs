pub use std::rc::{Rc, Weak};

mod light_elemental;
mod generic_minion;
mod generic_spell;
mod buff_card;

pub use self::light_elemental::*;
pub use self::generic_minion::*;
pub use self::generic_spell::*;
pub use self::buff_card::*;

pub trait Card {
    fn name(&self) -> &str;
    fn description(&self) -> &str {
        ""
    }
    fn cost(&self) -> Vec<(ResourceType, u8)>;
    fn attack(&self) -> Option<&u8> {
        None
    }
    fn health(&self) -> Option<&u8> {
        None
    }
    fn health_mut(&mut self) -> Option<&mut u8> {
        None
    }
    fn play_effects(&self) -> Vec<CardPlayEffect> {
        vec![CardPlayEffect::SummonMinion]
    }
    fn debug_text(&self) -> String {
        self.name().to_string()
    }

    fn clone_box(&self) -> Box<Card>;
}

#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Red,
    Blue,
    White,
    Black,
}

#[derive(Debug)]
pub enum CardPlayEffect {
    SummonMinion,
    Target(TargetType),
}

bitflags! {
    pub flags TargetType: u8 {
        const TARGET_SELF           = 0b0001,
        const TARGET_OPPONENT       = 0b0010,
        const TARGET_OWNMINION      = 0b0100,
        const TARGET_OPPONENTMINION = 0b1000,
        const TARGET_EVERYTHING     = TARGET_SELF.bits | TARGET_OPPONENT.bits | TARGET_OWNMINION.bits | TARGET_OPPONENTMINION.bits
    }
}
