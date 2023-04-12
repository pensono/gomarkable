use std::cmp::min;
use cgmath::{EuclideanSpace, Point2, vec2, Vector2};
use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{color, display_temp, dither_mode, DRAWING_QUANT_BIT, waveform_mode};
use libremarkable::framebuffer::{FramebufferDraw, FramebufferRefresh};
use crate::cgmath_extensions::Decomposable;
use crate::go;

pub struct BoardUi {
    size: usize,
    board_start: Point2<i32>,
    board_size: Vector2<i32>,
    square_size: Vector2<i32>,
    stone_radius: u32,
    line_width: u32,
}

impl BoardUi {
    pub fn new(size: usize, ctx: &ApplicationContext) -> BoardUi {
        let minimum_border = 100i32;
        let line_width = 3u32;
        let stone_gap = 2i32;

        let (screen_height, screen_width) = ctx.get_dimensions();
        let screen_size = vec2(screen_width as i32, screen_height as i32);

        let square_dimension = (min(screen_width, screen_height) as i32 - (minimum_border * 2) - line_width as i32) / (size - 1) as i32;
        let square_size = vec2(square_dimension, square_dimension);
        let stone_radius = ((square_dimension - stone_gap) / 2) as u32;

        let board_dimension = square_dimension * (size as i32 - 1) + line_width as i32;
        let board_size = vec2(board_dimension, board_dimension);
        let board_start = Point2::from_vec((screen_size - board_size) / 2);

        BoardUi {
            size,
            board_start,
            board_size,
            square_size,
            stone_radius,
            line_width,
        }
    }

    pub fn draw_board(self: &BoardUi, state: &go::BoardState, ctx: &mut ApplicationContext) {
        let fb = ctx.get_framebuffer_ref();

        // Draw the board
        fb.draw_rect(
            self.board_start,
            self.board_size.cast().unwrap(),
            self.line_width,
            color::BLACK,
        );

        for i in 1..(self.size-1) {
            // Draw the vertical lines
            let vertical_start = self.board_start + (self.square_size.x_component() * i as i32);
            fb.draw_line(vertical_start, vertical_start + self.board_size.y_component(), self.line_width, color::BLACK);

            // Draw the horizontal lines
            let horizontal_start = self.board_start + (self.square_size.y_component() * i as i32);
            fb.draw_line(horizontal_start, horizontal_start + self.board_size.x_component(), self.line_width, color::BLACK);
        }

        // Draw the stones
        for i in 0..self.size {
            for j in 0..self.size {
                let position = self.board_start + (self.square_size.x_component() * i as i32) + (self.square_size.y_component() * j as i32);
                // Both of these need aliasing!
                match state.board[i][j] {
                    Some(go::Player::Black) => {
                        fb.fill_circle(position.into(), self.stone_radius, color::BLACK);
                    }
                    Some(go::Player::White) => {
                        // Unfortunately, there's no draw with width, but this looks fine
                        fb.fill_circle(position.into(), self.stone_radius, color::BLACK);
                        fb.fill_circle(position.into(), self.stone_radius - self.line_width, color::WHITE);
                    }
                    None => {}
                }
            }
        }

        // Draw the last move
        if let Some((x, y)) = state.last_move {
            let position = self.board_start + (self.square_size.x_component() * x as i32) + (self.square_size.y_component() * y as i32);
            fb.draw_circle(position, self.stone_radius / 2, color::BLACK);
        }

        // Draw ko
        if let Some((x, y)) = state.ko {
            let center = self.board_start + (self.square_size.x_component() * x as i32) + (self.square_size.y_component() * y as i32);
            let size = vec2(self.stone_radius as i32, self.stone_radius as i32);
            fb.draw_rect(center - size / 2, size.cast().unwrap(), self.line_width, color::BLACK);
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
}