use std::cmp::min;
use std::fs::File;
use std::io::Read;
use cgmath::num_traits::real::Real;
use cgmath::{Point2, Vector2};
use libremarkable::framebuffer::common::color;
use libremarkable::framebuffer;
use rusttype::{point, Font, Scale};
use once_cell::sync::Lazy;

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