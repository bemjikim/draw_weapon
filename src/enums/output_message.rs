pub mod output_message {
    // ====== 공통 ======
    pub const BACK_OPTION: &str = "(0을 누르면 이전 화면으로 돌아갑니다)";
    pub const INVALID_INPUT: &str = "잘못된 입력입니다.";
    pub const EXIT_GAME: &str = "게임을 종료합니다.";

    // ====== 메인 메뉴 ======
    pub const MAIN_MENU_TITLE: &str = "\n===== ⚒ 대장장이 게임 ⚒ =====";
    pub const MAIN_MENU_OPTION_1: &str = "1. 강화 하러 가기";
    pub const MAIN_MENU_OPTION_2: &str = "2. 상점";
    pub const MAIN_MENU_OPTION_3: &str = "3. 나가기";

    // ====== 강화 관련 ======
    pub const NO_SWORD: &str = "가지고 있는 검이 없습니다.";
    pub const CHOOSE_SWORD: &str = "몇 번째 검을 강화하시겠습니까?: ";
    pub const ENHANCE_START: &str = "\n=== 강화 메뉴 ===";
    pub const ENHANCE_SUCCESS: &str = "✨ 강화 성공!";
    pub const ENHANCE_FAIL: &str = "💥 강화 실패...";
    pub const ENHANCE_LEVEL_UP: &str = "검의 강화 단계가 상승했습니다.";
    pub const ENHANCE_LEVEL_DOWN: &str = "검의 강화 단계가 하락했습니다.";
    pub const ENHANCE_DESTROYED: &str = "검이 파괴되었습니다...";

    // ====== 상점 관련 ======
    pub const SHOP_INTRO: &str = "\n=== 🏪 상점에 오신 것을 환영합니다 ===";
    pub const SHOP_BUY_SUCCESS: &str = "구매를 완료했습니다.";
    pub const SHOP_SELL_SUCCESS: &str = "판매를 완료했습니다.";
    pub const SHOP_NOT_ENOUGH_GOLD: &str = "골드가 부족합니다.";
    pub const SHOP_NOTHING_TO_SELL: &str = "판매할 검이 없습니다.";
}
