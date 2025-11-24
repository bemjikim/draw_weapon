use crate::models::sword::Sword;

pub struct PlayerRepository {
    pub gold: u32,
    pub swords: Vec<Sword>,
    pub protection_count: u32,
    pub blessing_count: u32,
}

impl PlayerRepository {
    pub fn new() -> Self {
        Self {
            gold: 1000,
            swords: vec![Sword::new("검", 500, 450)],
            protection_count: 1,
            blessing_count: 1,
        }
    }
}
