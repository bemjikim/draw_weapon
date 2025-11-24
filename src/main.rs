mod models;
mod repository;
mod service;
mod controller;

use controller::game_controller::GameController;

fn main() {
    println!("대장장이 게임을 시작합니다!");
    let mut controller = GameController::new();
    controller.run();
}
