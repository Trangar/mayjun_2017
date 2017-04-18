use super::{Card, CardPlayEffect, ResourceType, TARGET_OWNMINION, TARGET_OPPONENTMINION};

#[derive(Clone, Copy)]
pub struct BuffCard {

}

impl Card for BuffCard {
    fn name(&self) -> &str { "Buff card" }
    fn description(&self) -> &str { "Gives a minion +1/+1" }
    fn play_effects(&self) -> Vec<CardPlayEffect> {
        vec![CardPlayEffect::Target(TARGET_OWNMINION | TARGET_OPPONENTMINION)]
    }
    fn cost(&self) -> Vec<(ResourceType, u8)> {
        vec![
            (ResourceType::White, 1)
        ]
    }
    fn clone(&self) -> Self {
        Clone::clone(self)
    }
}
