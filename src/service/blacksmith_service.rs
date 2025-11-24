use rand::Rng;
use crate::models::{item::Item, sword::Sword};
use crate::repository::player_repository::PlayerRepository;

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

    pub fn enhance(&mut self, idx: usize, use_protection: bool, use_blessing: bool) {
        if idx >= self.repo.swords.len() {
            println!("잘못된 선택입니다.");
            return;
        }

        let mut sword = self.repo.swords[idx].clone();
        if sword.level >= 7 {
            println!("이미 최대 강화입니다.");
            return;
        }

        let mut rate = Self::success_rate_for_level(sword.level) as i32;
        if use_blessing && self.repo.blessing_count > 0 {
            rate += 5;
            self.repo.blessing_count -= 1;
            println!("강화축복부적을 사용했습니다. 성공률 +5%.");
        }

        if rate > 100 { rate = 100; }
        println!("{} 현재 강화 {}강 -> 성공률 {}%", sword.name, sword.level, rate);

        let roll = rand::thread_rng().gen_range(0..100);
        if roll < rate {
            sword.level += 1;
            println!("강화 성공! 이제 {}강 입니다.", sword.level);
            self.repo.swords[idx] = sword;
        } else {
            println!("강화 실패...");
            let destroy_chance = sword.level.saturating_mul(10);
            let destroy_roll = rand::thread_rng().gen_range(0..100);

            if use_protection && self.repo.protection_count > 0 {
                self.repo.protection_count -= 1;
                println!("파괴 방지권 사용: 아이템으로 파괴를 방지했습니다.");
            } else if destroy_roll < destroy_chance {
                println!("강화 실패로 장비가 파괴되었습니다...");
                self.repo.swords.remove(idx);
            } else {
                if sword.level > 0 {
                    sword.level -= 1;
                    println!("장비가 손상되어 1강 하락. 현재 {}강.", sword.level);
                    self.repo.swords[idx] = sword;
                } else {
                    println!("0강이라 하락 불가.");
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
                    println!("파괴 방지권을 구입했습니다.");
                } else { println!("골드가 부족합니다."); }
            }
            Item::Blessing => {
                let price = 300;
                if self.repo.gold >= price {
                    self.repo.gold -= price;
                    self.repo.blessing_count += 1;
                    println!("강화축복부적을 구입했습니다.");
                } else { println!("골드가 부족합니다."); }
            }
        }
    }

    pub fn buy_sword(&mut self, name: &str, base_price: u32) {
        if self.repo.gold >= base_price {
            self.repo.gold -= base_price;
            self.repo.swords.push(Sword { name: name.into(), level: 0, base_price });
            println!("{}을(를) 구입했습니다.", name);
        } else {
            println!("골드가 부족합니다.");
        }
    }

    pub fn sell_sword(&mut self, idx: usize) {
        if idx >= self.repo.swords.len() {
            println!("잘못된 선택입니다.");
            return;
        }
        let sword = self.repo.swords.remove(idx);
        let price = if sword.level == 0 {
            ((sword.base_price as f32) * 0.9) as u32
        } else {
            ((sword.base_price as f32) * (1.0 + (sword.level as f32 * 0.1))) as u32
        };
        self.repo.gold += price;
        println!("{} 판매 완료! +{}골드", sword.display(), price);
    }
}
