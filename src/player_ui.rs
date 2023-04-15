use std::string::String;
use cgmath::{Point2, point2, vec2, Vector2};
use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{color, display_temp, dither_mode, DRAWING_QUANT_BIT, mxcfb_rect, waveform_mode};
use libremarkable::framebuffer::{draw, FramebufferDraw, FramebufferRefresh, PartialRefreshMode};
use libremarkable::input::{InputEvent, MultitouchEvent};
use crate::{go, text};
use crate::go::Player;

pub struct PlayerUi {
    player: Player,
    player_name: String,
    name_position: Point2<i32>,
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

        let mut name_position = rect_start + vec2(padding + height as i32, (height as i32 - text_size as i32) / 2);

        PlayerUi {
            player,
            player_name: String::from(player_name),
            name_position,
            text_size,
            rect_start,
            rect_size,
        }
    }

    pub fn draw(self: &PlayerUi, state: &go::BoardState, ctx: &mut ApplicationContext) {
        let fb = ctx.get_framebuffer_ref();

        let rect_color = match state.current_player == self.player {
            true => color::GRAY(0x20),
            false => color::WHITE,
        };
        eprintln!("Drawing player UI for player {:?} with color {:?}", self.player, rect_color);
        fb.fill_rect(self.rect_start, self.rect_size, rect_color);

        text::draw_text(
            fb,
            self.name_position,
            self.text_size,
            self.player_name.as_str(),
            color::BLACK,
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
            waveform_mode::WAVEFORM_MODE_GLR16,
            display_temp::TEMP_USE_PAPYRUS,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            DRAWING_QUANT_BIT,
            false
        );
    }
}