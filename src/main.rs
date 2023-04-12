//! This example is a very basic drawing application. Marker can draw on the
//! screen (without tilt or pressure sensitivity) and Marker Plus can use the
//! eraser end to erase.
//!
//! Drawing is done in the framebuffer without any caching, so it's not possible
//! to save the results to file, zoom or pan, etc. There are also no GUI
//! elements or interactivity other than the pen.
//!
//! The new event loop design makes this type of application very easy to make.

use std::cmp::min;
use cgmath::{EuclideanSpace, point2, Point2, vec2, Vector2};
use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{
    color, display_temp, dither_mode, waveform_mode, DRAWING_QUANT_BIT,
};
use libremarkable::framebuffer::PartialRefreshMode;
use libremarkable::framebuffer::{FramebufferDraw, FramebufferRefresh};
use libremarkable::input::{InputEvent, WacomEvent, WacomPen};
use crate::cgmath_extensions::Decomposable;

mod go;
mod cgmath_extensions;
mod board_ui;

fn main() {
    let mut app = ApplicationContext::default();
    app.clear(true);

    let mut board_ui = board_ui::BoardUi::new(19, &app);
    let mut state = go::BoardState::new(19);

    board_ui.draw_board(&state, &mut app, true);

    app.start_event_loop(false, true, false, |ctx: &mut ApplicationContext, event: InputEvent| {
        if let InputEvent::MultitouchEvent { event, .. } = event {
            board_ui.handle_event(event, ctx, &mut state);
        }
    });
}
