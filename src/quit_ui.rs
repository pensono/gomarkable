use crate::drawing;
use crate::ui::{UiComponent, UiController};
use cgmath::{point2, vec2, Point2, Vector2};
use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{
    display_temp, dither_mode, mxcfb_rect, waveform_mode, DRAWING_QUANT_BIT,
};
use libremarkable::framebuffer::{FramebufferRefresh, PartialRefreshMode};
use libremarkable::image;
use libremarkable::image::RgbImage;
use libremarkable::input::{InputEvent, MultitouchEvent};
use std::cell::RefCell;
use std::rc::Rc;

pub struct QuitUi {
    image: RgbImage,
    position: Point2<i32>,
    size: Vector2<i32>,
}

impl QuitUi {
    pub fn new(ctx: &ApplicationContext) -> QuitUi {
        let (_screen_height, screen_width) = ctx.get_dimensions();
        let image = image::load_from_memory(include_bytes!("../assets/quit.png"))
            .unwrap()
            .to_rgb8();

        QuitUi {
            position: point2((screen_width - image.width()) as i32, 0),
            size: vec2(image.width() as i32, image.height() as i32),
            image,
        }
    }
}

impl<State> UiComponent<State> for QuitUi {
    fn handle_event(
        self: &mut QuitUi,
        _ui: Rc<RefCell<&mut UiController>>,
        _: &mut State,
        event: &InputEvent,
    ) {
        if let InputEvent::MultitouchEvent { event, .. } = event {
            if let MultitouchEvent::Release { finger } = event {
                if finger.pos.x >= self.position.x as u16 && finger.pos.y < self.size.y as u16 {
                    // TODO only exit scene
                    std::process::exit(0);
                }
            }
        }
    }

    fn draw(self: &QuitUi, ui: Rc<RefCell<&mut UiController>>, _: &State) {
        let fb = ui.borrow_mut().context.get_framebuffer_ref();

        // TODO add a white outline around the quit button
        drawing::draw_blended_image(fb, &self.image, self.position);

        let refresh_rect = mxcfb_rect {
            top: self.position.y as u32,
            left: self.position.x as u32,
            width: self.size.x as u32,
            height: self.size.y as u32,
        };

        fb.partial_refresh(
            &refresh_rect,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_DU,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            DRAWING_QUANT_BIT,
            false,
        );
    }
}
