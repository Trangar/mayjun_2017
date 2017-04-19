use super::{Card, ResourceType};

#[derive(Clone, Copy)]
pub struct LightElemental {
    pub health: u8,
}

impl Card for LightElemental {
    fn name(&self) -> &str {
        "Light elemental"
    }
    fn description(&self) -> &str {
        "Will always have the same\nattack as health."
    }
    fn attack(&self) -> Option<&u8> {
        Some(&self.health)
    }
    fn health(&self) -> Option<&u8> {
        Some(&self.health)
    }
    fn health_mut(&mut self) -> Option<&mut u8> {
        Some(&mut self.health)
    }
    fn debug_text(&self) -> String {
        format!("Light elemental ({}/{})", self.health, self.health)
    }
    fn cost(&self) -> Vec<(ResourceType, u8)> {
        vec![(ResourceType::White, 2)]
    }
    fn clone_box(&self) -> Box<Card> {
        Box::new(*self)
    }
}