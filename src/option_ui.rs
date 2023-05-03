use crate::text::TextAlignment;
use crate::ui::{UiComponent, UiController};
use crate::{drawing, text, ui};
use cgmath::{point2, vec2, Point2, Vector2};
use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{
    color, display_temp, dither_mode, mxcfb_rect, waveform_mode, DRAWING_QUANT_BIT,
};
use libremarkable::framebuffer::{FramebufferDraw, FramebufferRefresh, PartialRefreshMode};
use libremarkable::input::{InputEvent, MultitouchEvent};
use std::cell::RefCell;
use std::rc::Rc;
use std::string::String;

pub struct OptionUi<State> {
    option_names: Vec<String>,
    selected: usize,
    box_starts: Vec<Point2<i32>>,
    box_size: Vector2<u32>,
    size: Vector2<u32>,
    text_offset: Vector2<i32>,
    text_size: i32,
    title: String,
    title_position: Point2<i32>,
    callback: Box<dyn Fn(Rc<RefCell<&mut UiController>>, &mut State, &String)>,
}

impl<State> OptionUi<State> {
    pub fn new(
        ctx: &ApplicationContext,
        vertical_position: i32,
        title: String,
        option_names: Vec<String>,
        callback: Box<dyn Fn(Rc<RefCell<&mut UiController>>, &mut State, &String)>,
    ) -> OptionUi<State> {
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
        let box_size = vec2(
            (size.x - spacing * (option_names.len() - 1) as u32) / option_names.len() as u32,
            height,
        );

        let mut box_starts = Vec::new();
        for i in 0..option_names.len() {
            box_starts.push(point2(
                (minimum_border + (box_size.x + spacing) * i as u32) as i32,
                vertical_position,
            ));
        }

        let text_offset = vec2(
            box_size.x as i32 / 2i32,
            (height as i32 - text_size as i32) / 2,
        );

        let title_position = point2(minimum_border as i32, vertical_position) + title_offset;

        OptionUi {
            option_names,
            selected: 0,
            box_starts,
            box_size,
            size,
            text_offset,
            text_size,
            title,
            title_position,
            callback,
        }
    }
}

impl<State> UiComponent<State> for OptionUi<State> {
    fn handle_event(
        &mut self,
        ui: Rc<RefCell<&mut UiController>>,
        state: &mut State,
        event: &InputEvent,
    ) {
        if let InputEvent::MultitouchEvent { event, .. } = event {
            if let MultitouchEvent::Press { finger } = event {
                for i in 0..self.box_starts.len() {
                    let box_start = self.box_starts[i];
                    let box_end = box_start + self.box_size.cast().unwrap();
                    if finger.pos.x >= box_start.x as u16
                        && finger.pos.x < box_end.x as u16
                        && finger.pos.y >= box_start.y as u16
                        && finger.pos.y < box_end.y as u16
                    {
                        self.selected = i;
                        (self.callback)(ui.clone(), state, &self.option_names[self.selected]);

                        ui::post_redraw();
                    }
                }
            }
        }
    }

    fn draw(&self, ui: Rc<RefCell<&mut UiController>>, state: &State) {
        let fb = ui.borrow_mut().context.get_framebuffer_ref();

        text::draw_text(
            fb,
            self.title_position,
            TextAlignment::Left,
            self.text_size,
            color::BLACK,
            &self.title,
        );

        for i in 0..self.option_names.len() {
            if i == self.selected {
                fb.fill_rect(self.box_starts[i], self.box_size, color::BLACK);
                text::draw_text(
                    fb,
                    self.box_starts[i] + self.text_offset,
                    TextAlignment::Centered,
                    self.text_size,
                    color::WHITE,
                    &self.option_names[i],
                );
            } else {
                fb.fill_rect(self.box_starts[i], self.box_size, color::WHITE);
                drawing::draw_rect(fb, self.box_starts[i], self.box_size, 2);
                text::draw_text(
                    fb,
                    self.box_starts[i] + self.text_offset,
                    TextAlignment::Centered,
                    self.text_size,
                    color::BLACK,
                    &self.option_names[i],
                );
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
            false,
        );
    }
}
