use std::cmp::min;
use std::fs::File;
use std::io::Read;
use cgmath::num_traits::real::Real;
use cgmath::Point2;
use libremarkable::framebuffer::common::color;
use libremarkable::framebuffer;
use rusttype::{point, Font, Scale};
use once_cell::sync::Lazy;

pub static UI_FONT: Lazy<Font<'static>> = Lazy::new(|| {
    let mut file = File::open("/usr/share/fonts/ttf/noto/NotoSansUI-Bold.ttf").expect("Could not open ui font file");
    let mut file_data = vec![];
    file.read_to_end(&mut file_data).expect("Could not load ui font data");
    Font::try_from_vec(file_data)
        .expect("corrupted font data")
});

pub fn draw_text(
    fb: &mut dyn framebuffer::FramebufferIO,
    pos: Point2<i32>,
    text_size: i32,
    text: &str,
    col: color,
) {

    let scale_value = text_size as f32 * 2f32;

    let scale = Scale {
        x: scale_value,
        y: scale_value,
    };

    // The starting positioning of the glyphs (left middle)
    let start = point(pos.x as f32, pos.y as f32 + (scale_value / 2f32));

    let components = col.to_rgb8();
    let c1 = f32::from(255 - components[0]);
    let c2 = f32::from(255 - components[1]);
    let c3 = f32::from(255 - components[2]);

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in UI_FONT.layout(text, scale, start) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                let mult = (1.0 - v).min(1.0);
                let text_color = color::RGB((c1 * mult) as u8, (c2 * mult) as u8, (c3 * mult) as u8);
                let screen_position = Point2 {
                    x: (x + bounding_box.min.x as u32) as u32,
                    y: (y + bounding_box.min.y as u32) as u32,
                };

                let existing_color = fb.read_pixel(screen_position);
                let [e1, e2, e3] = existing_color.to_rgb8();
                let draw_color = color::RGB(min(e1, (c1 * mult) as u8), min(e2, (c2 * mult) as u8), min(e3, (c3 * mult) as u8));

                fb.write_pixel(screen_position.cast().unwrap(), draw_color);
            });
        }
    }
}
