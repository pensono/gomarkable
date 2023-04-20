use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{display_temp, dither_mode, DRAWING_QUANT_BIT, waveform_mode};
use libremarkable::framebuffer::FramebufferRefresh;
use libremarkable::input::InputEvent;
use crate::event_loop;

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

    pub fn draw(&self, ctx: &mut ApplicationContext, state: &State) {
        for component in &self.components {
            component.draw(ctx, state);
        }
    }

    pub fn start(&mut self, ctx: &mut ApplicationContext) {
        ctx.clear(true);
        self.draw(ctx, &state);
        ctx.get_framebuffer_ref().full_refresh(
            waveform_mode::WAVEFORM_MODE_GC16,
            display_temp::TEMP_USE_MAX,
            dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
            DRAWING_QUANT_BIT,
            true
        );

        ctx.start_event_loop(false, true, false, |ctx: &mut ApplicationContext, event: InputEvent| {
            for component in &self.components {
                component.handle_event(ctx, &mut self.state, &event);
            }

            while event_loop::needs_redraw() {
                event_loop::reset_redraw();
                self.draw(ctx, &state);
            }
        });
    }
}

pub trait UiComponent<State> {
    fn handle_event(&self, ctx: &mut ApplicationContext, state: &mut State, event: &InputEvent) {}
    fn draw(&self, ctx: &mut ApplicationContext, state: &State);
}