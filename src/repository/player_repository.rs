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
            swords: vec![Sword { name: "기본검".into(), level: 0, base_price: 500 }],
            protection_count: 1,
            blessing_count: 1,
        }
    }
}
