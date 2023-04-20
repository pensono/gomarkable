use libremarkable::appctx::ApplicationContext;
use crate::game_controller::GameController;
use crate::go::{BoardState, Player};

mod go;
mod cgmath_extensions;
mod board_ui;
mod player_ui;
mod text;
mod event_loop;
mod drawing;
mod ui;
mod quit_ui;
mod game_controller;
mod two_player_controller;

fn main() {
    let mut ctx = ApplicationContext::default();

    let mut scene = ui::Scene::new();
    scene.add(board_ui::BoardUi::new(&ctx, 19));
    scene.add(player_ui::PlayerUi::new(&ctx, "White", true, Player::White));
    scene.add(player_ui::PlayerUi::new(&ctx, "Black", false, Player::Black));
    scene.add(quit_ui::QuitUi::new(&ctx));

    let mut game_controller : Box<dyn GameController> = Box::new(two_player_controller::TwoPlayerController::new(BoardState::new(19)));

    scene.start(&mut ctx, &mut game_controller);
}