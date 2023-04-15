use std::fs::File;
use std::io::Read;
use std::sync::atomic::AtomicBool;
use cgmath::num_traits::real::Real;
use cgmath::Point2;
use libremarkable::framebuffer::common::color;
use libremarkable::framebuffer;
use rusttype::{point, Font, Scale};
use once_cell::sync::Lazy;

static NEEDS_REDRAW: AtomicBool = AtomicBool::new(false);

pub fn post_redraw() {
    NEEDS_REDRAW.store(true, std::sync::atomic::Ordering::SeqCst);
}

pub fn needs_redraw() -> bool {
    return NEEDS_REDRAW.load(std::sync::atomic::Ordering::SeqCst);
}

pub fn reset_redraw() {
    NEEDS_REDRAW.store(false, std::sync::atomic::Ordering::SeqCst);
}

