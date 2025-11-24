pub mod input_message {
    // ====== 공통 ======
    pub const INPUT_PROMPT: &str = "\n선택: ";
    pub const INVALID_INPUT: &str = "\n잘못된 입력입니다. 숫자를 다시 입력해주세요.";
    pub const MENU_MAIN: &str = "메뉴를 선택하세요 (1: 강화 / 2: 상점 / 3: 종료)";

    // ====== 강화 관련 ======
    pub const CHOOSE_SWORD: &str = "\n강화할 검의 번호를 입력하세요 (0을 눌러 이전으로): ";
    pub const CHOOSE_ENHANCE_TYPE: &str = "1. 그냥 강화하기  |  2. 아이템 사용하기 | 3. 이전";
    pub const CHOOSE_ITEM_PROTECT_OR_BLESS: &str =
        "\n[사용할 아이템 선택]\n1. 파괴 방지권 | 2. 강화축복부적 | 3. 둘 다 사용 | 4. 취소";

    // ====== 상점 관련 ======
    pub const SHOP_CHOICE: &str = "1. 구매하기  |  2. 판매하기  |  3. 이전";
    pub const CHOOSE_ITEM_SELL: &str = "\n판매할 검을 선택하세요 (0을 눌러 이전으로): ";
    pub const BUY_OPTIONS: &str = "1. 기본 검 (500)  |  2. 파괴 방지권(200)  |  3. 강화축복부적(300)  |  4. 이전";
}
