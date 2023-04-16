use libremarkable::appctx::ApplicationContext;
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

fn main() {
    let mut ctx = ApplicationContext::default();

    let mut scene = ui::Scene::new();
    scene.add(board_ui::BoardUi::new(&ctx, 19));
    scene.add(player_ui::PlayerUi::new(&ctx, "White", true, Player::White));
    scene.add(player_ui::PlayerUi::new(&ctx, "Black", false, Player::Black));
    scene.add(quit_ui::QuitUi::new(&ctx));

    let mut state = BoardState::new(19);

    scene.start(&mut ctx, &mut state);
}