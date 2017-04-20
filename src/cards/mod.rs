pub use std::rc::{Rc, Weak};

mod light_elemental;
mod generic_minion;
mod generic_spell;
mod buff_card;

pub use self::light_elemental::*;
pub use self::generic_minion::*;
pub use self::generic_spell::*;
pub use self::buff_card::*;

/// The basic card trait
/// This is the "contract" that binds card implementations to the system
/// Only name() and cost() are required, the others can be overwritten when needed
pub trait Card {
    /// Get the name of the card
    fn name(&self) -> &str;
    /// Get the cost of the card
    fn cost(&self) -> Vec<(ResourceType, u8)>;
    /// Get the description of the card, if any
    fn description(&self) -> &str {
        ""
    }
    /// Get the attack of the card, if any
    /// Minions should always have an attack value
    fn attack(&self) -> Option<&u8> {
        None
    }
    /// Get the health of the card, if any
    /// Minions should always have a health value
    /// If this is 0 or None, the minion gets destroyed
    fn health(&self) -> Option<&u8> {
        None
    }
    /// Mutable reference to the health
    /// See health(&self) for more info
    fn health_mut(&mut self) -> Option<&mut u8> {
        None
    }
    /// The effects that this card has when you try to play it from hand
    /// This can either be summoning a minion, or targetting a minion or player, or a combination of this
    fn play_effects(&self) -> Vec<CardPlayEffect> {
        vec![CardPlayEffect::SummonMinion]
    }
    /// A debug text that's used to describe this card for logging purposes
    fn debug_text(&self) -> String {
        self.name().to_string()
    }
    /// Clone this object into a boxed copy of itself
    fn clone_box(&self) -> Box<Card>;
}

/// The resource type that the game has
/// Cards will require a certain amount of resources played. See Card::cost
/// Players will be able to add 1 ResourceType to their pool every turn
/// Cards will consume these resources, and they'll recharge the next turn
/// Leaving the player with the same amount of total resources as they've had turns
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    Red,
    Blue,
    White,
    Black,
}

/// The effect that a card has when it gets played
#[derive(Debug)]
pub enum CardPlayEffect {
    /// The card gets summoned as a minion
    SummonMinion,
    /// The card targets something. This can be a combination of the player, the players minions, the opponents minions and the opponent
    Target(TargetType),
}

bitflags! {
    /// Flags that are used to indicate what a card can target. These values can be or'd together to form combinations.
    pub flags TargetType: u8 {
        const TARGET_SELF           = 0b0001,
        const TARGET_OPPONENT       = 0b0010,
        const TARGET_OWNMINION      = 0b0100,
        const TARGET_OPPONENTMINION = 0b1000,
        const TARGET_EVERYTHING     = TARGET_SELF.bits | TARGET_OPPONENT.bits | TARGET_OWNMINION.bits | TARGET_OPPONENTMINION.bits
    }
}
