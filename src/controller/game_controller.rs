use std::io::{self, Write};
use crate::repository::player_repository::PlayerRepository;
use crate::service::blacksmith_service::BlacksmithService;
use crate::models::item::Item;

use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::thread;
use std::time::Duration;

use crate::enums::input_message::input_message::*;
use crate::enums::output_message::output_message::*;

pub struct GameController {
    repo: PlayerRepository,
}

impl GameController {
    pub fn new() -> Self {
        Self {
            repo: PlayerRepository::new(),
        }
    }

    pub fn run(&mut self) {
        // ===== 음악 재생 =====
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let filepath = "assets\\sound\\menu.mp3";
        let abspath = Path::new(env!("CARGO_MANIFEST_DIR")).join(filepath);

        let file = File::open(abspath).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source.repeat_infinite());
        sink.set_volume(0.3);

        // ===== 메인 루프 =====
        loop {
            clear_screen();
            println!("{}", TITLE_MAIN);
            println!("{}", CLEAR_LINE);
            println!("{}", MENU_MAIN);

            print!("{}", INPUT_PROMPT);
            io::stdout().flush().unwrap();

            match read_trimmed_line().as_deref() {
                Some("1") => self.enhance_menu(),
                Some("2") => self.shop_menu(),
                Some("3") => {
                    println!("게임을 종료합니다.");
                    break;
                }
                _ => println!("{}", INVALID_INPUT),
            }
        }
    }

    // =============================
    // 강화 메뉴
    // =============================
    fn enhance_menu(&mut self) {
        loop {
            clear_screen();

            if self.repo.swords.is_empty() {
                println!("{}", NO_SWORD);
                screen_loading();
                return;
            }

            println!("{}", ENHANCE_MENU_TITLE);
            println!("{}", CURRENT_SWORD_LIST);

            for (i, s) in self.repo.swords.iter().enumerate() {
                println!("{}. {}", i + 1, s.display());
            }

            print!("{}", CHOOSE_SWORD);

            io::stdout().flush().unwrap();

            let input = read_trimmed_line();
            if input.as_deref() == Some("0") {
                return;
            }

            let idx = match input.and_then(|s| s.parse::<usize>().ok()) {
                Some(n) if (1..=self.repo.swords.len()).contains(&n) => n - 1,
                _ => {
                    println!("{}", INVALID_INPUT);
                    continue;
                }
            };

            self.enhance_item_menu(idx);
        }
    }

    // =============================
    // 강화 방식 선택
    // =============================
    fn enhance_item_menu(&mut self, idx: usize) {
        clear_screen();

        println!("{} {}", SELECTED_SWORD, self.repo.swords[idx].display());

        println!("{}", CHOOSE_ENHANCE_TYPE);
        print!("{}", INPUT_PROMPT);
        io::stdout().flush().unwrap();

        clear_screen();
        match read_trimmed_line().as_deref() {
            Some("1") => {
                BlacksmithService { repo: &mut self.repo }.enhance(idx, false, false);
            }
            Some("2") => {
                println!("{}", CHOOSE_ITEM_PROTECT_OR_BLESS);
                print!("{}{}", PROTECTING_ITEM_STATUS, self.repo.protection_count);
                println!("{}{}" , BLESSING_ITEM_STATUS, self.repo.blessing_count);
                io::stdout().flush().unwrap();
                match read_trimmed_line().as_deref() {
                    Some("1") => BlacksmithService { repo: &mut self.repo }.enhance(idx, true, false),
                    Some("2") => BlacksmithService { repo: &mut self.repo }.enhance(idx, false, true),
                    Some("3") => BlacksmithService { repo: &mut self.repo }.enhance(idx, true, true),
                    _ => println!("취소되었습니다."),
                }
            }
            _ => return,
        }

        print!("{}", ENHANCE_COMPLETED);
        screen_loading();
    }

    // =============================
    // 상점 메뉴
    // =============================
    fn shop_menu(&mut self) {
        loop {
            clear_screen();
            println!("{}", SHOP_TITLE);
            println!("{}", SHOP_CHOICE);

            print!("{}", INPUT_PROMPT);
            io::stdout().flush().unwrap();

            match read_trimmed_line().as_deref() {
                Some("1") => self.shop_buy_menu(),
                Some("2") => self.shop_sell_menu(),
                Some("3") => return,
                _ => println!("{}", INVALID_INPUT),
            }
        }
    }

    // =============================
    // 상점 - 구매
    // =============================
    fn shop_buy_menu(&mut self) {
        loop {
            clear_screen();

            println!("{}", SHOP_BUY_TITLE);
            println!("{} {}" , GOLD_STATUS, self.repo.gold);
            println!("{}", BUY_OPTIONS);

            print!("{}", INPUT_PROMPT);
            io::stdout().flush().unwrap();

            match read_trimmed_line().as_deref() {
                Some("1") => BlacksmithService { repo: &mut self.repo }.buy_sword("검", 500),
                Some("2") => BlacksmithService { repo: &mut self.repo }.buy_item(Item::Protection),
                Some("3") => BlacksmithService { repo: &mut self.repo }.buy_item(Item::Blessing),
                Some("4") => return,
                _ => println!("{}", INVALID_INPUT),
            }

            screen_loading();
        }
    }

    // =============================
    // 상점 - 판매
    // =============================
    fn shop_sell_menu(&mut self) {
        if self.repo.swords.is_empty() {
            println!("{}", SELL_EMPTY);
            return;
        }

        loop {
            clear_screen();
            println!("{}", SHOP_SELL_TITLE);

            for (i, s) in self.repo.swords.iter().enumerate() {
                println!("{}. {} ({}원)", i + 1, s.display(), s.display_price);
            }

            print!("{}", CHOOSE_ITEM_SELL);

            io::stdout().flush().unwrap();
            
            match read_trimmed_line().and_then(|s| s.parse::<usize>().ok()) {
                Some(0) => return,
                Some(n) if (1..=self.repo.swords.len()).contains(&n) => {
                    BlacksmithService { repo: &mut self.repo }.sell_sword(n - 1)
                }
                _ => println!("{}", INVALID_INPUT),
            }

            screen_loading();
        }
    }
}

// =============================
// 입력 공통
// =============================
pub fn read_trimmed_line() -> Option<String> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let s = input.trim().to_string();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    } else {
        None
    }
}

// =============================
// 화면 지우기
// =============================
fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}


// =============================
// 화면 로딩
// =============================
fn screen_loading() {
    println!("{}", SCREEN_LOADING);
    thread::sleep(Duration::from_secs(3));
}