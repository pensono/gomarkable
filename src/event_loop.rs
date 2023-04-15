use std::sync::atomic::AtomicBool;

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

