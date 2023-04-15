use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{
    color, display_temp, dither_mode, waveform_mode, DRAWING_QUANT_BIT,
};
use libremarkable::framebuffer::{FramebufferRefresh, PartialRefreshMode};
use libremarkable::input::{InputEvent};
use crate::cgmath_extensions::Decomposable;
use crate::go::Player;

mod go;
mod cgmath_extensions;
mod board_ui;
mod player_ui;
mod text;
mod event_loop;

fn main() {
    let mut app = ApplicationContext::default();
    app.clear(true);

    let mut board_ui = board_ui::BoardUi::new(19, &app);
    let mut white_ui = player_ui::PlayerUi::new("White", true, Player::White, &app);
    let mut black_ui = player_ui::PlayerUi::new("Black", false, Player::Black, &app);
    let mut state = go::BoardState::new(19);

    board_ui.draw(&state, &mut app);
    white_ui.draw(&state, &mut app);
    black_ui.draw(&state, &mut app);

    app.get_framebuffer_ref().full_refresh(
        waveform_mode::WAVEFORM_MODE_GC16,
        display_temp::TEMP_USE_MAX,
        dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
        DRAWING_QUANT_BIT,
        true
    );

    app.start_event_loop(false, true, false, |ctx: &mut ApplicationContext, event: InputEvent| {
        if let InputEvent::MultitouchEvent { event, .. } = event {
            board_ui.handle_event(event, ctx, &mut state);
        }

        while event_loop::needs_redraw() {
            event_loop::reset_redraw();

            board_ui.draw(&state, ctx);
            white_ui.draw(&state, ctx);
            black_ui.draw(&state, ctx);
        }
    });
}
