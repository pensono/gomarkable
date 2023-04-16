use std::fmt::format;
use std::string::String;
use cgmath::{Point2, point2, vec2, Vector2};
use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{color, display_temp, dither_mode, DRAWING_QUANT_BIT, mxcfb_rect, waveform_mode};
use libremarkable::framebuffer::{draw, FramebufferDraw, FramebufferIO, FramebufferRefresh, PartialRefreshMode};
use libremarkable::input::{InputEvent, MultitouchEvent};
use crate::{drawing, go, text};
use crate::cgmath_extensions::Decomposable;
use crate::go::Player;

pub struct PlayerUi {
    player: Player,
    player_name: String,
    name_position: Point2<i32>,
    captures_position: Point2<i32>,
    text_size: i32,
    rect_start: Point2<i32>,
    rect_size: Vector2<u32>,
}

impl PlayerUi {
    pub fn new(player_name: &str, top: bool, player: Player, ctx: &ApplicationContext) -> PlayerUi {
        let height = 104u32;
        let text_size = 18i32;
        let padding = 10i32;

        let (screen_height, screen_width) = ctx.get_dimensions();

        let mut rect_start = point2(0, 0);
        let rect_size = vec2(screen_width as u32, height);
        if !top {
            rect_start.y += screen_height as i32 - height as i32
        }

        let text_topline = (height as i32 - text_size as i32) / 2;
        let name_position = rect_start + vec2(padding + height as i32, text_topline);
        let captures_position = rect_start + vec2(screen_width as i32 - padding - height as i32, text_topline);

        PlayerUi {
            player,
            player_name: String::from(player_name),
            name_position,
            captures_position,
            text_size,
            rect_start,
            rect_size,
        }
    }

    pub fn draw(self: &PlayerUi, state: &go::BoardState, ctx: &mut ApplicationContext) {
        let fb = ctx.get_framebuffer_ref();

        if state.current_player == self.player {
            // Use a dithered rectangle so that the update can be drawn using the direct update waveform
            drawing::dithered_fill_rect(fb, self.rect_start, self.rect_size, 8, 3);
        } else {
            fb.fill_rect(self.rect_start, self.rect_size, color::WHITE);
        }

        text::draw_text(
            fb,
            self.name_position,
            text::TextAlignment::Left,
            self.text_size,
            self.player_name.as_str()
        );

        let captures = state.captures_made_by(self.player);
        let mut captures_string = match captures {
            1 => format!("{} Capture", captures),
            _ => format!("{} Captures", captures)
        };

        if self.player == Player::White {
            captures_string = format!("{}.5 Komi  {}", state.komi_minus_half, captures_string);
        }

        text::draw_text(
            fb,
            self.captures_position,
            text::TextAlignment::Right,
            self.text_size,
            captures_string.as_str()
        );

        let refresh_rect = mxcfb_rect {
            top: self.rect_start.y as u32,
            left: self.rect_start.x as u32,
            width: self.rect_size.x as u32,
            height: self.rect_size.y as u32,
        };

        fb.partial_refresh(
            &refresh_rect,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_DU,
            display_temp::TEMP_USE_PAPYRUS,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            DRAWING_QUANT_BIT,
            false
        );
    }

    pub fn cleanup(self: &PlayerUi, state: &go::BoardState, ctx: &mut ApplicationContext) {
        let fb = ctx.get_framebuffer_ref();

        let refresh_rect = mxcfb_rect {
            top: self.rect_start.y as u32,
            left: self.rect_start.x as u32,
            width: self.rect_size.x as u32,
            height: self.rect_size.y as u32,
        };

        fb.partial_refresh(
            &refresh_rect,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_REAGL,
            display_temp::TEMP_USE_PAPYRUS,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            DRAWING_QUANT_BIT,
            false
        );
    }
}