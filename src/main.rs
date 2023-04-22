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
mod option_ui;

fn main() {
    let mut ctx = ApplicationContext::default();

    let mut game_options = GameOptions {
        board_size: 19,
        clock: 0,
    };
    let mut menu = ui::Scene::new(game_options);
    menu.add(option_ui::OptionUi::new(&ctx, 400i32, "", vec!["1-Player", "2-Player", "OGS"], |game_options: &mut GameOptions, value: &str| {}));
    menu.add(option_ui::OptionUi::new(&ctx, 400i32 + 200*1, "Board Size" , vec!["9x9", "13x13", "19x19"], |game_options: &mut GameOptions, value: &str| {
        game_options.board_size = match value {
            "9" => 9,
            "13" => 13,
            "19" => 19,
            _ => 19,
        };
    }));
    menu.add(option_ui::OptionUi::new(&ctx, 400i32 + 200*2, "Clock", vec!["None", "Rapid", "Blitz"], |game_options: &mut GameOptions, value: &str| { }));
    menu.add(option_ui::OptionUi::new(&ctx, 400i32 + 200*3, "Handicap", vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"], |game_options: &mut GameOptions, value: &str| { }));
    menu.add(option_ui::OptionUi::new(&ctx, 1400i32, "", vec!["Play"], |game_options: &mut GameOptions, value: &str| { }));

    let game_controller : Box<dyn GameController> = Box::new(two_player_controller::TwoPlayerController::new(BoardState::new(19)));
    let mut gameplay = ui::Scene::new(game_controller);
    gameplay.add(board_ui::BoardUi::new(&ctx, 19));
    gameplay.add(player_ui::PlayerUi::new(&ctx, "White", true, Player::White));
    gameplay.add(player_ui::PlayerUi::new(&ctx, "Black", false, Player::Black));
    gameplay.add(quit_ui::QuitUi::new(&ctx));

    let mut current_scene = menu;

    ctx.clear(true);
    current_scene.draw(&mut ctx);
    ctx.get_framebuffer_ref().full_refresh(
        waveform_mode::WAVEFORM_MODE_INIT,
        display_temp::TEMP_USE_MAX,
        dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
        DRAWING_QUANT_BIT,
        true
    );

    ctx.start_event_loop(false, true, false, |ctx: &mut ApplicationContext, event: InputEvent| {
        current_scene.handle_event(ctx, event);
    });
}

pub struct GameOptions {
    board_size: usize,
    clock: u32,
}