use std::cmp::min;
use std::fs::File;
use std::io::Read;
use cgmath::num_traits::real::Real;
use cgmath::Point2;
use libremarkable::framebuffer::common::color;
use libremarkable::framebuffer;
use rusttype::{point, Font, Scale};
use once_cell::sync::Lazy;
use crate::drawing;

pub static UI_FONT: Lazy<Font<'static>> = Lazy::new(|| {
    let mut file = File::open("/usr/share/fonts/ttf/noto/NotoSansUI-Bold.ttf").expect("Could not open ui font file");
    let mut file_data = vec![];
    file.read_to_end(&mut file_data).expect("Could not load ui font data");
    Font::try_from_vec(file_data)
        .expect("corrupted font data")
});

pub enum TextAlignment {
    Left,
    Centered,
    Right,
}

pub fn draw_text(
    fb: &mut dyn framebuffer::FramebufferIO,
    pos: Point2<i32>,
    alignment: TextAlignment,
    text_size: i32,
    text: &str,
) {
    let scale = scale_for_size(text_size);

    let alignment_offset = match alignment {
        TextAlignment::Left => 0f32,
        TextAlignment::Centered => text_width(text_size, text) as f32 / 2f32,
        TextAlignment::Right => text_width(text_size, text) as f32,
    };

    let start = point(pos.x as f32 - alignment_offset, pos.y as f32 + text_size as f32);

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in UI_FONT.layout(text, scale, start) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                let screen_position = Point2 {
                    x: (x + bounding_box.min.x as u32) as u32,
                    y: (y + bounding_box.min.y as u32) as u32,
                };

                let text_color = color::GRAY((255.0 * v) as u8);
                let existing_color = fb.read_pixel(screen_position);
                let draw_color = drawing::darkest(existing_color, text_color);

                fb.write_pixel(screen_position.cast().unwrap(), draw_color);
            });
        }
    }
}

pub fn text_width(text_size: i32, text: &str) -> i32 {
    if text.len() == 0 {
        return 0;
    }

    let scale = scale_for_size(text_size);
    let last_glyph = UI_FONT.layout(text, scale, point(0.0, 0.0)).last().unwrap();

    return last_glyph.pixel_bounding_box().unwrap().max.x;
}


fn scale_for_size(text_size: i32) -> Scale {
    // 2 is a magic value for noto UI? unsure what's going on but it works
    let scale_value = text_size as f32 * 2f32;

    return Scale {
        x: scale_value,
        y: scale_value,
    };
}