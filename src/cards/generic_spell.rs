use super::{Card, CardPlayEffect, ResourceType, TARGET_EVERYTHING};

/// A generic spell that deals 3 damage to a targetted minion or player
#[derive(Clone, Copy)]
pub struct DamageSpellCard {}

impl Card for DamageSpellCard {
    fn name(&self) -> &str {
        "Damage spell card"
    }
    fn description(&self) -> &str {
        "Deal 3 damage to a target"
    }
    fn play_effects(&self) -> Vec<CardPlayEffect> {
        vec![CardPlayEffect::Target(TARGET_EVERYTHING)]
    }
    fn cost(&self) -> Vec<(ResourceType, u8)> {
        vec![(ResourceType::Red, 2)]
    }
    fn clone_box(&self) -> Box<Card> {
        Box::new(*self)
    }
}
