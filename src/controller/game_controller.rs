use std::io::{self, Write};
use crate::repository::player_repository::PlayerRepository;
use crate::service::blacksmith_service::BlacksmithService;
use crate::models::item::Item;

pub struct GameController {
    repo: PlayerRepository,
}

impl GameController {
    pub fn new() -> Self {
        Self { repo: PlayerRepository::new() }
    }

    pub fn run(&mut self) {
        loop {
            clear_screen();
            println!("\n=== 대장장이 게임 ===");
            println!("메뉴를 선택하세요 (1: 강화 / 2: 상점 / 3: 종료, 0: 이전)");
            print!("선택: "); io::stdout().flush().unwrap();

            match read_trimmed_line().as_deref() {
                Some("1") => self.enhance_menu(),
                Some("2") => self.shop_menu(),
                Some("3") => { println!("게임을 종료합니다."); break; }
                _ => println!("잘못된 입력입니다."),
            }
        }
    }

    fn enhance_menu(&mut self) {
        loop {
            clear_screen();
            if self.repo.swords.is_empty() {
                println!("현재 소지한 검이 없습니다.");
                return;
            }

            println!("\n-- 강화 하기 --");
            println!("현재 보유한 검 목록:");
            for (i, s) in self.repo.swords.iter().enumerate() {
                println!("{}. {}", i+1, s.display());
            }
            
            
            print!("\n강화할 검의 번호를 입력하세요 (0: 이전)" ); io::stdout().flush().unwrap();
            
            let input = read_trimmed_line();
            if input.as_deref() == Some("0") { return; }
            clear_screen();
            let idx = match input.and_then(|s| s.parse::<usize>().ok()) {
                Some(n) if n>=1 && n<=self.repo.swords.len() => n-1,
                _ => { println!("잘못된 선택입니다."); continue; }
            };

            println!("1. 그냥 강화\n2. 아이템 사용 (파괴방지권: {} | 강화축복부적: {})\n3. 취소" , self.repo.protection_count, self.repo.blessing_count);
            print!("선택: "); io::stdout().flush().unwrap();
            clear_screen();
            match read_trimmed_line().as_deref() {
                Some("1") => BlacksmithService { repo: &mut self.repo }.enhance(idx, false, false),
                Some("2") => {
                    println!("\n[아이템 선택]\n1. 파괴 방지권: {}\n2. 강화축복부적: {}\n3. 둘 다\n4. 취소" , self.repo.protection_count, self.repo.blessing_count);
                    print!("선택: "); io::stdout().flush().unwrap();
                    match read_trimmed_line().as_deref() {
                        Some("1") => BlacksmithService { repo: &mut self.repo }.enhance(idx, true, false),
                        Some("2") => BlacksmithService { repo: &mut self.repo }.enhance(idx, false, true),
                        Some("3") => BlacksmithService { repo: &mut self.repo }.enhance(idx, true, true),
                        _ => println!("취소"),
                    }
                }
                _ => println!("취소"),
            }
        }
    }

    fn shop_menu(&mut self) {
        loop {
            println!("\n-- 상점 --");
            println!("1. 구입하기\n2. 판매하기\n3. 이전");
            print!("선택: "); io::stdout().flush().unwrap();

            match read_trimmed_line().as_deref() {
                Some("1") => self.shop_buy_menu(),
                Some("2") => self.shop_sell_menu(),
                Some("3") => return,
                _ => println!("잘못된 입력입니다."),
            }
        }
    }

    fn shop_buy_menu(&mut self) {
        loop {
            println!("\n-- 구입하기 --");
            println!("골드: {}", self.repo.gold);
            println!("1. 검 (기본검 500)\n2. 파괴 방지권(200)\n3. 강화축복부적(300)\n4. 이전");
            print!("선택: "); io::stdout().flush().unwrap();

            match read_trimmed_line().as_deref() {
                Some("1") => BlacksmithService { repo: &mut self.repo }.buy_sword("기본 검", 500),
                Some("2") => BlacksmithService { repo: &mut self.repo }.buy_item(Item::Protection),
                Some("3") => BlacksmithService { repo: &mut self.repo }.buy_item(Item::Blessing),
                Some("4") => return,
                _ => println!("잘못된 입력입니다."),
            }
        }
    }

    fn shop_sell_menu(&mut self) {
        if self.repo.swords.is_empty() {
            println!("판매할 검이 없습니다.");
            return;
        }

        loop {
            println!("\n-- 판매하기 --");
            for (i, s) in self.repo.swords.iter().enumerate() {
                println!("{}. {} (원가 {})", i+1, s.display(), s.base_price);
            }
            println!("(0을 눌러 이전)");
            print!("판매할 검 번호: "); io::stdout().flush().unwrap();

            match read_trimmed_line().and_then(|s| s.parse::<usize>().ok()) {
                Some(0) => return,
                Some(n) if n>=1 && n<=self.repo.swords.len() =>
                    BlacksmithService { repo: &mut self.repo }.sell_sword(n-1),
                _ => println!("잘못된 입력입니다."),
            }
        }
    }
}

pub fn read_trimmed_line() -> Option<String> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let s = input.trim().to_string();
        if s.is_empty() { None } else { Some(s) }
    } else { None }
}


fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}