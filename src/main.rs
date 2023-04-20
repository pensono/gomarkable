use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{display_temp, dither_mode, DRAWING_QUANT_BIT, waveform_mode};
use libremarkable::framebuffer::FramebufferRefresh;
use libremarkable::input::InputEvent;
use crate::game_controller::GameController;
use crate::go::{BoardState, Player};

mod go;
mod cgmath_extensions;
mod board_ui;
mod player_ui;
mod text;
mod drawing;
mod ui;
mod quit_ui;
mod game_controller;
mod two_player_controller;

fn main() {
    let mut ctx = ApplicationContext::default();

    let game_controller : Box<dyn GameController> = Box::new(two_player_controller::TwoPlayerController::new(BoardState::new(19)));
    let mut scene = ui::Scene::new(game_controller);
    scene.add(board_ui::BoardUi::new(&ctx, 19));
    scene.add(player_ui::PlayerUi::new(&ctx, "White", true, Player::White));
    scene.add(player_ui::PlayerUi::new(&ctx, "Black", false, Player::Black));
    scene.add(quit_ui::QuitUi::new(&ctx));

    let mut current_scene = scene;

    current_scene.draw(&mut ctx);
    ctx.get_framebuffer_ref().full_refresh(
        waveform_mode::WAVEFORM_MODE_GC16,
        display_temp::TEMP_USE_MAX,
        dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
        DRAWING_QUANT_BIT,
        true
    );

    ctx.start_event_loop(false, true, false, |ctx: &mut ApplicationContext, event: InputEvent| {
        current_scene.handleEvent(ctx, event);
    });
}