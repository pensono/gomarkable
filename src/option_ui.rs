use std::string::String;
use cgmath::{Point2, point2, vec2, Vector2};
use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{color, display_temp, dither_mode, DRAWING_QUANT_BIT, mxcfb_rect, waveform_mode};
use libremarkable::framebuffer::{FramebufferDraw, FramebufferRefresh, PartialRefreshMode};
use libremarkable::input::{InputEvent, MultitouchEvent};
use crate::{drawing, GameOptions, text, ui};
use crate::text::TextAlignment;
use crate::ui::UiComponent;

pub struct OptionUi {
    option_names: Vec<String>,
    selected: usize,
    box_starts: Vec<Point2<i32>>,
    box_size: Vector2<u32>,
    size: Vector2<u32>,
    text_offset: Vector2<i32>,
    text_size: i32,
    title: String,
    title_position: Point2<i32>,
    callback: fn(&mut GameOptions, &str),
}

impl OptionUi {
    pub fn new(ctx: &ApplicationContext, vertical_position: i32, title: &str, option_names: Vec<&str>, callback: fn(&mut GameOptions, &str)) -> OptionUi {
        let minimum_border = 250;
        let title_offset = vec2(30, -50);
        let height = 80;
        let spacing = match option_names.len() {
            _ if option_names.len() > 4 => 20u32,
            _ => 60u32,
        };
        let text_size = 18;

        let (_, screen_width) = ctx.get_dimensions();

        let size = vec2(screen_width - minimum_border * 2 as u32, height);
        let box_size = vec2((size.x  - spacing * (option_names.len() - 1) as u32) / option_names.len() as u32, height);

        let mut box_starts = Vec::new();
        for i in 0..option_names.len() {
            box_starts.push(point2((minimum_border + (box_size.x + spacing) * i as u32) as i32, vertical_position ));
        }

        let text_offset = vec2(box_size.x as i32 / 2i32, (height as i32 - text_size as i32) / 2);

        let title_position = point2(minimum_border as i32, vertical_position) + title_offset;

        OptionUi {
            option_names: option_names.iter().map(|s| s.to_string()).collect(),
            selected: 0,
            box_starts,
            box_size,
            size,
            text_offset,
            text_size,
            title: title.to_string(),
            title_position,
            callback,
        }
    }
}

impl UiComponent<GameOptions> for OptionUi {
    fn handle_event(self: &mut OptionUi, ctx: &mut ApplicationContext, state: &mut GameOptions, event: &InputEvent) {
        if let InputEvent::MultitouchEvent { event, .. } = event {
            if let MultitouchEvent::Press { finger } = event
            {
                for i in 0..self.box_starts.len() {
                    let box_start = self.box_starts[i];
                    let box_end = box_start + self.box_size.cast().unwrap();
                    if finger.pos.x >= box_start.x as u16 && finger.pos.x < box_end.x as u16 && finger.pos.y >= box_start.y as u16 && finger.pos.y < box_end.y as u16 {
                        self.selected = i;
                        (self.callback)(state, &*self.option_names[self.selected]);

                        ui::post_redraw();
                    }
                }
            }
        }
    }

    fn draw(self: &OptionUi, ctx: &mut ApplicationContext, state: &GameOptions) {
        let fb = ctx.get_framebuffer_ref();

        text::draw_text(fb, self.title_position, TextAlignment::Left, self.text_size, color::BLACK, &self.title);

        for i in 0..self.option_names.len() {
            if i == self.selected {
                fb.fill_rect(self.box_starts[i],  self.box_size, color::BLACK);
                text::draw_text(fb, self.box_starts[i] + self.text_offset, TextAlignment::Centered, self.text_size, color::WHITE, &self.option_names[i]);
            } else {
                fb.fill_rect(self.box_starts[i],  self.box_size, color::WHITE);
                drawing::draw_rect(fb, self.box_starts[i],  self.box_size, 2);
                text::draw_text(fb, self.box_starts[i] + self.text_offset, TextAlignment::Centered, self.text_size, color::BLACK, &self.option_names[i]);
            }
        }

        let refresh_rect = mxcfb_rect {
            top: self.box_starts[0].y as u32,
            left: self.box_starts[0].x as u32,
            width: self.size.x as u32,
            height: self.size.y as u32,
        };

        // TODO increase responsiveness by first drawing with DU waveform, then refreshing
        // the screen with GC16
        fb.partial_refresh(
            &refresh_rect,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_GC16_FAST,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            DRAWING_QUANT_BIT,
            false
        );
    }
}