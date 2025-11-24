mod models;
mod repository;
mod service;
mod controller;
mod enums;

use controller::game_controller::GameController;

fn main() {
    let mut controller = GameController::new();
    controller.run();
}
