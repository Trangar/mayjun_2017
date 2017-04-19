use super::{Card, ResourceType};

pub struct GenericMinion {
    pub name: String,
    pub attack: u8,
    pub health: u8,
    pub cost: Vec<(ResourceType, u8)>,
}

impl Card for GenericMinion {
    fn name(&self) -> &str {
        &self.name
    }
    fn attack(&self) -> Option<&u8> {
        Some(&self.attack)
    }
    fn health(&self) -> Option<&u8> {
        Some(&self.health)
    }
    fn health_mut(&mut self) -> Option<&mut u8> {
        Some(&mut self.health)
    }
    fn debug_text(&self) -> String {
        format!("{} ({}/{})", self.name(), self.attack, self.health)
    }
    fn cost(&self) -> Vec<(ResourceType, u8)> {
        self.cost.clone()
    }
    fn clone_box(&self) -> Box<Card> {
        Box::new(GenericMinion {
                     name: self.name.clone(),
                     attack: self.attack,
                     health: self.health,
                     cost: self.cost.clone(),
                 })
    }
}
