use cgmath::point2;
use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{
    color, display_temp, dither_mode, waveform_mode, DRAWING_QUANT_BIT,
};
use libremarkable::framebuffer::{FramebufferDraw, FramebufferRefresh, PartialRefreshMode};
use libremarkable::image;
use libremarkable::image::GenericImageView;
use libremarkable::input::{InputEvent, MultitouchEvent};
use crate::cgmath_extensions::Decomposable;
use crate::go::{BoardState, Player};

mod go;
mod cgmath_extensions;
mod board_ui;
mod player_ui;
mod text;
mod event_loop;
mod drawing;

fn main() {
    let mut ctx = ApplicationContext::default();

    let board_ui = board_ui::BoardUi::new(19, &ctx);
    let white_ui = player_ui::PlayerUi::new("White", true, Player::White, &ctx);
    let black_ui = player_ui::PlayerUi::new("Black", false, Player::Black, &ctx);
    let mut state = BoardState::new(19);

    let quit_image = image::load_from_memory(include_bytes!("../assets/quit.png")).unwrap();

    let redraw = |ctx: &mut ApplicationContext, state: &BoardState| {
        board_ui.draw(&state, ctx);
        white_ui.draw(&state, ctx);
        black_ui.draw(&state, ctx);

        // TODO add a white outline around the quit button
        drawing::draw_blended_image(ctx.get_framebuffer_ref(), &quit_image.to_rgb8(), point2(1404 - 100, 0));

        white_ui.cleanup(&state, ctx);
        black_ui.cleanup(&state, ctx);
    };

    ctx.clear(true);
    redraw(&mut ctx, &state);
    ctx.get_framebuffer_ref().full_refresh(
        waveform_mode::WAVEFORM_MODE_GC16,
        display_temp::TEMP_USE_MAX,
        dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
        DRAWING_QUANT_BIT,
        true
    );

    ctx.start_event_loop(false, true, false, |ctx: &mut ApplicationContext, event: InputEvent| {
        if let InputEvent::MultitouchEvent { event, .. } = event {
            board_ui.handle_event(event, ctx, &mut state);
        }

        // Quick exit button implementation
        if let InputEvent::MultitouchEvent { event, .. } = event {
            if let MultitouchEvent::Release { finger, .. } = event {
                if finger.pos.x > 1404 - 104 && finger.pos.y < 104 {
                    std::process::exit(0);
                }
            }
        }

        while event_loop::needs_redraw() {
            event_loop::reset_redraw();
            redraw(ctx, &state);
        }
    });
}