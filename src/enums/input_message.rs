pub mod input_message {
    // ====== 공통 ======
    pub const INPUT_PROMPT: &str = "입력 >> ";
    pub const INVALID_INPUT: &str = "잘못된 입력입니다. 숫자를 다시 입력해주세요.";
    pub const BACK_OPTION: &str = "(0을 누르면 이전 화면으로 돌아갑니다)";

    // ====== 강화 관련 ======
    pub const CHOOSE_SWORD: &str = "어떤 검을 강화하시겠습니까?";
    pub const CHOOSE_ENHANCE_TYPE: &str = "1. 그냥 강화하기  |  2. 아이템 사용하기";
    pub const CHOOSE_ITEM_USE: &str = "사용할 아이템을 선택하세요 (0: 없음)";

    // ====== 상점 관련 ======
    pub const SHOP_CHOICE: &str = "1. 구매하기  |  2. 판매하기";
    pub const CHOOSE_ITEM_BUY: &str = "구매할 아이템을 선택하세요:";
    pub const CHOOSE_ITEM_SELL: &str = "판매할 검을 선택하세요:";
}