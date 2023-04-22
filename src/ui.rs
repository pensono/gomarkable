use libremarkable::appctx::ApplicationContext;
use libremarkable::input::InputEvent;
use std::sync::atomic::AtomicBool;

pub struct Scene<State> {
    components: Vec<Box<dyn UiComponent<State>>>,
    state: State,
}


impl<State> Scene<State> {
    pub fn new(initial_state: State) -> Scene<State> {
        Scene {
            components: Vec::new(),
            state: initial_state,
        }
    }

    pub fn add<C: UiComponent<State> + 'static>(&mut self, component: C) {
        self.components.push(Box::new(component));
    }

    pub fn draw(&self, ctx: &mut ApplicationContext) {
        for component in &self.components {
            component.draw(ctx, &self.state);
        }
    }

    pub fn handle_event(&mut self, ctx: &mut ApplicationContext, event: InputEvent) {
        for component in self.components.iter_mut() {
            component.handle_event(ctx, &mut self.state, &event);
        }

        while needs_redraw() {
            reset_redraw();
            self.draw(ctx);
        }
    }
}

pub trait UiComponent<State> {
    fn handle_event(&mut self, ctx: &mut ApplicationContext, state: &mut State, event: &InputEvent) {}
    fn draw(&self, ctx: &mut ApplicationContext, state: &State);
}


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

