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

fn main() {
    let mut app = ApplicationContext::default();

    app.clear(true);

    let mut state = go::BoardState::new(19);
    state.board[3][3] = Option::from(go::Player::Black);
    state.board[3][4] = Option::from(go::Player::Black);
    state.board[9][9] = Option::from(go::Player::White);
    state.board[12][9] = Option::from(go::Player::White);
    state.last_move = Option::from((9, 9));
    state.ko = Option::from((4, 9));
    draw_board(&state, &mut app);

    app.start_event_loop(true, false, false, &handle_event);
}

fn draw_board(state: &go::BoardState, ctx: &mut ApplicationContext) {
    let fb = ctx.get_framebuffer_ref();
    let size = state.size;

    let (screen_height, screen_width) = ctx.get_dimensions();
    let screen_size = vec2(screen_width as i32, screen_height as i32);

    let minimum_border = 100i32;
    let line_width = 3u32;
    let stone_gap = 2i32;
    let square_dimension = (min(screen_width, screen_height) as i32 - (minimum_border * 2) - line_width as i32) / (size - 1) as i32;
    let square_size = vec2(square_dimension, square_dimension);
    let stone_radius = ((square_dimension - stone_gap) / 2) as u32;

    let board_dimension = square_dimension * (size as i32 - 1) + line_width as i32;
    let board_size = vec2(board_dimension, board_dimension);
    let board_start = Point2::from_vec((screen_size - board_size) / 2);

    // Draw the board
    fb.draw_rect(
        board_start,
        board_size.cast().unwrap(),
        line_width,
        color::BLACK,
    );

    for i in 1..(size-1) {
        // Draw the vertical lines
        let vertical_start = board_start + (square_size.x_component() * i as i32);
        fb.draw_line(vertical_start, vertical_start + board_size.y_component(), line_width, color::BLACK);

        // Draw the horizontal lines
        let horizontal_start = board_start + (square_size.y_component() * i as i32);
        fb.draw_line(horizontal_start, horizontal_start + board_size.x_component(), line_width, color::BLACK);
    }

    // Draw the stones
    for i in 0..size {
        for j in 0..size {
            let position = board_start + (square_size.x_component() * i as i32) + (square_size.y_component() * j as i32);
            // Both of these need aliasing!
            match state.board[i][j] {
                Some(go::Player::Black) => {
                    fb.fill_circle(position.into(), stone_radius, color::BLACK);
                }
                Some(go::Player::White) => {
                    // Unfortunately, there's no draw with width, but this looks fine
                    fb.fill_circle(position.into(), stone_radius, color::BLACK);
                    fb.fill_circle(position.into(), stone_radius - line_width, color::WHITE);
                }
                None => {}
            }
        }
    }

    // Draw the last move
    if let Some((x, y)) = state.last_move {
        let position = board_start + (square_size.x_component() * x as i32) + (square_size.y_component() * y as i32);
        fb.draw_circle(position, stone_radius / 2, color::BLACK);
    }

    // Draw ko
    if let Some((x, y)) = state.ko {
        let center = board_start + (square_size.x_component() * x as i32) + (square_size.y_component() * y as i32);
        let size = vec2(stone_radius as i32, stone_radius as i32);
        fb.draw_rect(center - size / 2, size.cast().unwrap(), line_width, color::BLACK);
    }

    fb.full_refresh(
        waveform_mode::WAVEFORM_MODE_GC16,
        display_temp::TEMP_USE_MAX,
        dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
        DRAWING_QUANT_BIT,
        true
    );

    eprintln!("Refreshed!");
}

fn handle_event(ctx: &mut ApplicationContext, event: InputEvent) {
    eprintln!("Event: {:?}", event);
}