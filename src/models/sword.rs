#[derive(Clone)]
pub struct Sword {
    pub name: String,
    pub level: u8, // 0..=7
    pub base_price: u32,
}

impl Sword {
    pub fn display(&self) -> String {
        format!("[{}, {}강]", self.name, self.level)
    }
}
