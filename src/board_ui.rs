use std::cmp::min;
use cgmath::{Array, ElementWise, EuclideanSpace, Point2, point2, vec2, Vector2};
use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{color, display_temp, dither_mode, DRAWING_QUANT_BIT, mxcfb_rect, waveform_mode};
use libremarkable::framebuffer::{FramebufferDraw, FramebufferRefresh, PartialRefreshMode};
use libremarkable::input::{InputEvent, MultitouchEvent};
use crate::cgmath_extensions::Decomposable;
use crate::{event_loop, go};
use crate::go::BoardState;
use crate::ui::UiComponent;

pub struct BoardUi {
    size: usize,
    board_start: Point2<i32>,
    board_size: Vector2<i32>,
    square_size: Vector2<i32>,
    hoshi_radius: u32,
    line_width: u32,
    stone_radius: u32,
}

fn hoshi_points(size: usize) -> Vec<Point2<usize>> {
    match size {
        19 => vec![
            point2(3, 3),point2(9, 3),point2(15, 3),
            point2(3, 9),point2(9, 9),point2(15, 9),
            point2(3, 15),point2(9, 15),point2(15, 15),
        ],
        13 => vec![
            point2(3, 3),point2(10, 3),
            point2(6, 6),
            point2(3, 10),point2(10, 10),
        ],
        9 => vec![
            point2(2, 2), point2(6, 2),
            point2(4, 4),
            point2(2, 6), point2(6, 6),
        ],
        _ => vec![]
    }
}

impl BoardUi {
    pub fn new(ctx: &ApplicationContext, size: usize) -> BoardUi {
        let minimum_border = match size {
            size if size > 13 => 100i32,
            _ => 150i32,
        };

        let line_width = 3u32;
        let stone_gap = 2i32;
        let star_radius = 8u32;

        let (screen_height, screen_width) = ctx.get_dimensions();
        let screen_size = vec2(screen_width as i32, screen_height as i32);

        let square_dimension = (min(screen_width, screen_height) as i32 - (minimum_border * 2) - line_width as i32) / (size - 1) as i32;
        let square_size = Vector2::from_value(square_dimension);
        let stone_radius = ((square_dimension - stone_gap) / 2) as u32;

        let board_dimension = square_dimension * (size as i32 - 1) + line_width as i32;
        let board_size = Vector2::from_value(board_dimension);
        let board_start = Point2::from_vec((screen_size - board_size) / 2);

        BoardUi {
            size,
            board_start,
            board_size,
            square_size,
            hoshi_radius: star_radius,
            line_width,
            stone_radius,
        }
    }

    fn board_to_screen(self: &BoardUi, point: Point2<usize>) -> Point2<i32> {
        self.board_start + (self.square_size.x_component() * point.x as i32) + (self.square_size.y_component() * point.y as i32)
    }
}

impl UiComponent<BoardState> for BoardUi {
    fn handle_event(self: &BoardUi, ctx: &mut ApplicationContext, state: &mut BoardState, event: &InputEvent) {
        if let InputEvent::MultitouchEvent { event, .. } = event {
            // TODO show a ghost square on press/move, and play on release
            if let MultitouchEvent::Press { finger } = event
            {
                let board_position = Point2::from_vec(finger.pos.cast().unwrap() - self.board_start);
                let point = Point2::from_vec((board_position + (self.square_size / 2)).to_vec().div_element_wise(self.square_size));

                if point.x >= 0 && point.x < self.size as i32 && point.y >= 0 && point.y < self.size as i32 {
                    let legal_move = state.try_play(point.cast().unwrap()).is_ok();

                    if legal_move {
                        event_loop::post_redraw();
                    }
                }
            }
        }
    }

    fn draw(self: &BoardUi, ctx: &mut ApplicationContext, state: &go::BoardState) {
        // TODO when a piece is played, only redraw the parts which changed. Could be more responsive?

        let fb = ctx.get_framebuffer_ref();

        // Board background. This is important for if any pieces are removed since we never fully clear the screen
        fb.fill_rect(
            self.board_start - self.square_size,
            (self.board_size + self.square_size * 2).cast().unwrap(),
            color::WHITE
        );

        // Draw the board outline
        fb.draw_rect(
            self.board_start,
            self.board_size.cast().unwrap(),
            self.line_width,
            color::BLACK,
        );

        for i in 1..(self.size-1) {
            // Draw the vertical lines
            let vertical_start = self.board_to_screen(point2(i, 0));
            fb.draw_line(vertical_start, vertical_start + self.board_size.y_component(), self.line_width, color::BLACK);

            // Draw the horizontal lines
            let horizontal_start = self.board_to_screen(point2(0, i));
            fb.draw_line(horizontal_start, horizontal_start + self.board_size.x_component(), self.line_width, color::BLACK);
        }

        // Draw star points
        for point in hoshi_points(self.size) {
            fb.fill_circle(self.board_to_screen(point), self.hoshi_radius, color::BLACK);
        }

        // Draw the stones
        for x in 0..self.size {
            for y in 0..self.size {
                let position = self.board_to_screen(point2(x, y));
                // TODO Both of these need aliasing!
                match state.board[x][y] {
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
        if let Some(point) = state.last_move {
            let color = match state.current_player {
                go::Player::Black => color::WHITE,
                go::Player::White => color::BLACK,
            };
            fb.draw_circle(self.board_to_screen(point), self.stone_radius / 2, color);
        }

        // Draw ko
        if let Some(point) = state.ko {
            let center = self.board_to_screen(point);
            let size = vec2(self.stone_radius as i32, self.stone_radius as i32);
            fb.draw_rect(center - size / 2, size.cast().unwrap(), self.line_width / 2, color::BLACK);
        }

        let refresh_rect = mxcfb_rect {
            top: (self.board_start.y - self.square_size.y) as u32,
            left: (self.board_start.x - self.square_size.x) as u32,
            width: (self.board_size.x + self.square_size.x * 2) as u32,
            height: (self.board_size.y + self.square_size.y * 2) as u32,
        };

        fb.partial_refresh(
            &refresh_rect,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_DU,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            DRAWING_QUANT_BIT,
            false
        );
    }
}