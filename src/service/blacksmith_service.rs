use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use rand::Rng;

use crate::models::{item::Item, sword::Sword};
use crate::repository::player_repository::PlayerRepository;
use crate::enums::output_message::output_message::*;

pub struct BlacksmithService<'repo_life_time> {
    pub repo: &'repo_life_time mut PlayerRepository,
}

impl<'repo_life_time> BlacksmithService<'repo_life_time> {
    fn success_rate_for_level(level: u8) -> u8 {
        match level {
            0 => 100, 1 => 90, 2 => 80, 3 => 70,
            4 => 60, 5 => 50, 6 => 30, 7 => 10,
            _ => 0,
        }
    }

    fn name_prefix(level: u8) -> &'static str {
        match level {
            0 => "기본 ",
            1 => "예리한 ",
            2 => "단단한 ",
            3 => "빛나는 ",
            4 => "신화의 ",
            5 => "전설의 ",
            6 => "초월한 ",
            7 => "신들의 ",
            _ => "",
        }
    }

    pub fn enhance(&mut self, idx: usize, use_protection: bool, use_blessing: bool) {

        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let base_filepath = "assets\\sound\\upgrade_sound.mp3";
        let success_filepath = "assets\\sound\\success.mp3";
        let fail_filepath = "assets\\sound\\fail.mp3";

        let base_abspath = Path::new(env!("CARGO_MANIFEST_DIR")).join(base_filepath);   
        let success_abspath = Path::new(env!("CARGO_MANIFEST_DIR")).join(success_filepath);
        let fail_abspath = Path::new(env!("CARGO_MANIFEST_DIR")).join(fail_filepath);

        let base_file = File::open(base_abspath).unwrap();
        let success_file = File::open(success_abspath).unwrap();
        let fail_file = File::open(fail_abspath).unwrap();

        if idx >= self.repo.swords.len() {
            println!("{}", INVALID_INDEX);
            return;
        }

        let mut sword = self.repo.swords[idx].clone();
        if sword.level >= 7 {
            println!("{}", MAX_LEVEL);
            return;
        }

        let mut rate = Self::success_rate_for_level(sword.level) as i32;
        if use_blessing && self.repo.blessing_count > 0 {
            rate += 5;
            self.repo.blessing_count -= 1;

            println!("{}", USE_BLESSING);
        }


        if rate > 100 { rate = 100; }
        println!("{} {} {}강 -> {} {}%", sword.display_name, CURRENT_STATUS_PREFIX, sword.level, SUCCESS_RATE_PREFIX, rate);

        let source = Decoder::new(BufReader::new(base_file)).unwrap();
        sink.append(source);
        sink.set_volume(8.0);
        sink.sleep_until_end();

        let roll = rand::thread_rng().gen_range(0..100);
        if roll < rate {
            println!("\n{}", ENHANCE_SUCCESS);
            let source = Decoder::new(BufReader::new(success_file)).unwrap();
            sink.append(source);
            sink.set_volume(0.4);
            sink.sleep_until_end();

            sword.level += 1;
            let prefix = Self::name_prefix(sword.level);

            sword.display_name = format!("{}{}", prefix, sword.base_name);
            sword.display_price = BlacksmithService::calculate_sword_price(&sword);

            println!("\n{} 이제 {}강 입니다.",{ENHANCE_SUCCESS}, sword.level);
            self.repo.swords[idx] = sword;
            
        } else {
            println!("\n{}", ENHANCE_FAIL);
            let destroy_chance = sword.level.saturating_mul(10);
            let destroy_roll = rand::thread_rng().gen_range(0..100);
            let source = Decoder::new(BufReader::new(fail_file)).unwrap();
            sink.append(source);
            sink.set_volume(8.0);
            sink.sleep_until_end();

            if use_protection && self.repo.protection_count > 0 {
                self.repo.protection_count -= 1;
                println!("{}", DESTROY_PREVENTED);
            } else if destroy_roll < destroy_chance {
                println!("{}", DESTROYED);
                self.repo.swords.remove(idx);
            } else {
                if sword.level > 0 {
                    sword.level -= 1;
                    let prefix = Self::name_prefix(sword.level);
                    
                    sword.display_name = format!("{}{}", prefix, sword.base_name);
                    sword.display_price = BlacksmithService::calculate_sword_price(&sword);
                    
                    println!("{} 현재 {}강.", LEVEL_DOWN, sword.level);
                    self.repo.swords[idx] = sword;
                    
                } else {
                    println!("{}", LEVEL_DOWN_IMPOSSIBLE);
                }
            }
        }
    }

    pub fn buy_item(&mut self, item: Item) {
        match item {
            Item::Protection => {
                let price = 200;
                if self.repo.gold >= price {
                    self.repo.gold -= price;
                    self.repo.protection_count += 1;
                    println!("{}", BUY_PROTECTION);
                } else { println!("{}", GOLD_NOT_ENOUGH); }
            }
            Item::Blessing => {
                let price = 300;
                if self.repo.gold >= price {
                    self.repo.gold -= price;
                    self.repo.blessing_count += 1;
                    println!("{}", BUY_BLESSING);
                } else { println!("{}", GOLD_NOT_ENOUGH); }
            }
        }
    }

    pub fn buy_sword(&mut self, name: &str, base_price: u32) {
        if self.repo.gold >= base_price {
            self.repo.gold -= base_price;
            self.repo.swords.push(Sword::new(name, base_price, base_price));
            println!("{}{}", name, BUY_SWORD);
        } else {
            println!("{}", GOLD_NOT_ENOUGH);
        }
    }

    pub fn sell_sword(&mut self, idx: usize) {
        if idx >= self.repo.swords.len() {
            println!("{}", INVALID_INDEX);
            return;
        }
        let sword = self.repo.swords.remove(idx);
        let price = Self::calculate_sword_price(&sword);
        self.repo.gold += price;
        println!("{} {} {}골드", sword.display(), SELL_SWORD, price);
    }

    pub fn calculate_sword_price(sword: &Sword) -> u32 {
        let price = if sword.level == 0 {
            ((sword.base_price as f32) * 0.9) as u32
        } else {
            ((sword.base_price as f32) * (1.0 + (sword.level as f32 * 0.2))) as u32
        };

        return price;
    }
}
