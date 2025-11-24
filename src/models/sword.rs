#[derive(Clone)]
pub struct Sword {
    pub base_name: String,
    pub display_name: String,
    pub level: u8, // 0..=7
    pub base_price: u32,
    pub display_price: u32,
}

impl Sword {
    pub fn new(base_name: &str, base_price: u32, display_price: u32) -> Self {
        Self {
            base_name: base_name.into(),
            display_name: base_name.into(),
            level: 0,
            base_price,
            display_price,
        }
    }

    pub fn display(&self) -> String {
        format!("[{}, {}강]", self.display_name, self.level)
    }
}
