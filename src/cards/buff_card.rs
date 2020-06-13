use super::{Card, CardPlayEffect, ResourceType, TargetType};

/// A generic buff card that gives a minion 1 attack and 1 health
/// This can be cast on your own minions or the opponent minions
#[derive(Clone, Copy)]
pub struct BuffCard {}

impl Card for BuffCard {
    fn name(&self) -> &str {
        "Buff card"
    }
    fn description(&self) -> &str {
        "Gives a minion +1/+1"
    }
    fn play_effects(&self) -> Vec<CardPlayEffect> {
        vec![CardPlayEffect::Target(
            TargetType::TARGET_OWNMINION | TargetType::TARGET_OPPONENTMINION,
        )]
    }
    fn cost(&self) -> Vec<(ResourceType, u8)> {
        vec![(ResourceType::White, 1)]
    }
    fn clone_box(&self) -> Box<dyn Card> {
        Box::new(*self)
    }
}
