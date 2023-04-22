use std::cmp::{max, min};
use cgmath::{Point2, vec2, Vector2};
use libremarkable::framebuffer::common::color;
use libremarkable::framebuffer;
use libremarkable::image::{RgbImage};

pub fn dithered_fill_rect(fb: &mut dyn framebuffer::FramebufferIO, pos: Point2<i32>, size: Vector2<u32>, modulo: i32, offset: i32) {
    for ypos in pos.y..pos.y + size.y as i32 {
        for xpos in pos.x..pos.x + size.x as i32 {
            let color = match (xpos + ypos * offset) % modulo {
                0 => color::BLACK,
                _ => color::WHITE,
            };

            fb.write_pixel(
                Point2 {
                    x: xpos as i32,
                    y: ypos as i32,
                },
                color,
            );
        }
    }
}

pub fn draw_blended_image(fb: &mut dyn framebuffer::FramebufferIO, img: &RgbImage, pos: Point2<i32>) {
    for (x, y, pixel) in img.enumerate_pixels() {
        let pixel_pos = pos + vec2(x as u32, y as u32).cast().unwrap();
        let existing_color = fb.read_pixel(pixel_pos.cast().unwrap());

        let [r, g, b] = pixel.0;
        let image_color = color::RGB(r, g, b);
        let draw_color = darkest(existing_color, image_color);

        fb.write_pixel(pixel_pos, draw_color);
    }
}

#[inline]
pub fn darkest(a: color, b: color) -> color {
    let a_rgb = a.to_rgb8();
    let b_rgb = b.to_rgb8();
    return color::RGB(min(a_rgb[0], b_rgb[0]), min(a_rgb[1], b_rgb[1]), min(a_rgb[2], b_rgb[2]));
}

#[inline]
pub fn lightest(a: color, b: color) -> color {
    let a_rgb = a.to_rgb8();
    let b_rgb = b.to_rgb8();
    return color::RGB(max(a_rgb[0], b_rgb[0]), max(a_rgb[1], b_rgb[1]), max(a_rgb[2], b_rgb[2]));
}

#[inline]
pub fn multiply_f32(a: color, b: f32) -> color {
    let a_rgb = a.to_rgb8();
    return color::RGB((a_rgb[0] as f32 * b) as u8, (a_rgb[1] as f32 * b) as u8, (a_rgb[2] as f32 * b) as u8);
}

#[inline]
pub fn multiply_u8(a: color, b: u8) -> color {
    let a_rgb = a.to_rgb8();
    return color::RGB(a_rgb[0] * b / 255, a_rgb[1] * b / 255, a_rgb[2] * b / 255);
}