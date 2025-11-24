use crate::enums::output_message::output_message;
use crate::model::sword::Sword;

pub struct OutputView;

impl OutputView {
    pub fn print_main_menu() {
        println!("{}", output_message::MAIN_MENU_TITLE);
        println!("{}", output_message::MAIN_MENU_OPTION_1);
        println!("{}", output_message::MAIN_MENU_OPTION_2);
        println!("{}", output_message::MAIN_MENU_OPTION_3);
        println!("{}", output_message::BACK_OPTION);
    }

    pub fn print_swords(swords: &Vec<Sword>) {
        if swords.is_empty() {
            println!("{}", output_message::NO_SWORD);
            return;
        }

        println!("{}", output_message::CHOOSE_SWORD);
        println!("\n=== 검 목록 ===");
        for (i, sword) in swords.iter().enumerate() {
            println!("{}. {} ({}강)", i + 1, sword.name, sword.enhance_level);
        }
        println!("{}", output_message::BACK_OPTION);
    }

    pub fn print_exit() {
        println!("{}", output_message::EXIT_GAME);
    }

    pub fn print_invalid() {
        println!("{}", output_message::INVALID_INPUT);
    }
}
