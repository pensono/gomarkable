use std::cell::RefCell;
use std::rc::Rc;
use libremarkable::appctx::ApplicationContext;
use libremarkable::input::InputEvent;
use std::sync::atomic::AtomicBool;
use libremarkable::framebuffer::common::{display_temp, dither_mode, DRAWING_QUANT_BIT, waveform_mode};
use libremarkable::framebuffer::FramebufferRefresh;

pub struct UiController<'a> {
    pub context: ApplicationContext<'a>,
    pub current_scene: Rc<RefCell<dyn SceneTrait>>
}

impl<'a> UiController<'a> {
    pub fn new(context: ApplicationContext, initial_scene: Rc<RefCell<dyn SceneTrait>>) -> UiController {
        UiController {
            context,
            current_scene: initial_scene,
        }
    }

    pub fn change_scene(self_: Rc<RefCell<&mut Self>>, new_scene: Rc<RefCell<dyn SceneTrait>>) {
        let mut self_ref = self_.borrow_mut();
        self_ref.current_scene = new_scene;
        drop(self_ref);
        UiController::full_refresh(self_);
    }

    fn full_refresh(self_: Rc<RefCell<&mut Self>>) {
        self_.borrow_mut().context.clear(true);
        let mut scene = self_.clone().borrow_mut().current_scene.clone();
        scene.borrow_mut().draw(self_.clone());
        self_.borrow_mut().context.get_framebuffer_ref().full_refresh(
            waveform_mode::WAVEFORM_MODE_INIT,
            display_temp::TEMP_USE_MAX,
            dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
            DRAWING_QUANT_BIT,
            true
        );
    }

    pub fn start(self_: Rc<RefCell<&mut Self>>) {
        UiController::full_refresh(self_.clone());

        let context = self_.borrow_mut().context.upgrade_ref();
        context.start_event_loop(false, true, false, |ctx: &mut ApplicationContext, event: InputEvent| {
            let mut scene = self_.clone().borrow_mut().current_scene.clone();
            scene.borrow_mut().handle_event(self_.clone(), event);
        });
    }
}

pub struct Scene<State: ?Sized>
{
    components: Vec<Box<dyn UiComponent<State>>>,
    state: Box<State>,
}

pub trait SceneTrait {
    fn draw(&self, ui: Rc<RefCell<&mut UiController>>);
    fn handle_event(&mut self, ui: Rc<RefCell<&mut UiController>>, event: InputEvent);
}

impl<State> Scene<State> {
    pub fn new(initial_state: State) -> Scene<State> {
        Scene {
            components: Vec::new(),
            state: Box::new(initial_state),
        }
    }

    pub fn add<C: UiComponent<State> + 'static>(&mut self, component: C) {
        self.components.push(Box::new(component));
    }
}

impl<State> SceneTrait for Scene<State> {
    fn draw(&self, ui: Rc<RefCell<&mut UiController>>) {
        for component in &self.components {
            component.draw(ui.clone(), &self.state);
        }
    }

    fn handle_event(&mut self, ui: Rc<RefCell<&mut UiController>>, event: InputEvent) {
        for component in self.components.iter_mut() {
            component.handle_event(ui.clone(), &mut self.state, &event);
        }

        while needs_redraw() {
            reset_redraw();
            self.draw(ui.clone());
        }
    }
}

pub trait UiComponent<State: ?Sized> {
    fn handle_event(&mut self, ui: Rc<RefCell<&mut UiController>>, state: &mut State, event: &InputEvent) {}
    fn draw(&self, ui: Rc<RefCell<&mut UiController>>, state: &State);
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

