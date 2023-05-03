use libremarkable::appctx::ApplicationContext;
use libremarkable::framebuffer::common::{display_temp, dither_mode, waveform_mode, DRAWING_QUANT_BIT, mxcfb_rect};
use libremarkable::framebuffer::{FramebufferDraw, FramebufferRefresh, PartialRefreshMode};
use libremarkable::input::InputEvent;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;

pub struct UiController<'a> {
    pub context: ApplicationContext<'a>,
    pub current_scene: Rc<RefCell<dyn SceneTrait>>,
    pending_scene_change: bool,
    pending_scene_change_deep_refresh: bool,
}

impl<'a> UiController<'a> {
    pub fn new(
        context: ApplicationContext,
        initial_scene: Rc<RefCell<dyn SceneTrait>>,
    ) -> UiController {
        UiController {
            context,
            current_scene: initial_scene,
            pending_scene_change: false,
            pending_scene_change_deep_refresh: false,
        }
    }

    pub fn change_scene(self_: Rc<RefCell<&mut Self>>, new_scene: Rc<RefCell<dyn SceneTrait>>, deep_refresh: bool) {
        self_.borrow_mut().current_scene = new_scene;
        self_.borrow_mut().current_scene.borrow_mut().initialize();
        self_.borrow_mut().pending_scene_change = true;
        self_.borrow_mut().pending_scene_change_deep_refresh = deep_refresh;
    }

    fn full_refresh(self_: Rc<RefCell<&mut Self>>) {
        self_.borrow_mut().context.get_framebuffer_ref().clear();

        let scene = self_.clone().borrow_mut().current_scene.clone();
        scene.borrow_mut().draw(self_.clone());

        self_
            .borrow_mut()
            .context
            .get_framebuffer_ref()
            .full_refresh(
                waveform_mode::WAVEFORM_MODE_INIT,
                display_temp::TEMP_USE_MAX,
                dither_mode::EPDC_FLAG_USE_REMARKABLE_DITHER,
                DRAWING_QUANT_BIT,
                true,
            );

        reset_redraw();
    }

    fn partial_refresh(self_: Rc<RefCell<&mut Self>>) {
        self_.borrow_mut().context.get_framebuffer_ref().clear();

        let scene = self_.clone().borrow_mut().current_scene.clone();
        scene.borrow_mut().draw(self_.clone());

        let (screen_height, screen_width) = self_
            .borrow_mut()
            .context.get_dimensions();
        let screen_rect = mxcfb_rect {
            top: 0,
            left: 0,
            width: screen_height,
            height: screen_width,
        };
        self_
            .borrow_mut()
            .context
            .get_framebuffer_ref()
            .partial_refresh(
                &screen_rect,
                PartialRefreshMode::Async,
                waveform_mode::WAVEFORM_MODE_GC16_FAST,
                display_temp::TEMP_USE_REMARKABLE_DRAW,
                dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                DRAWING_QUANT_BIT,
                false,
            );

        reset_redraw();
    }

    pub fn start(self_: Rc<RefCell<&mut Self>>) {
        self_.borrow_mut().current_scene.borrow_mut().initialize();
        UiController::full_refresh(self_.clone());

        let context = self_.borrow_mut().context.upgrade_ref();
        context.start_event_loop(
            false,
            true,
            false,
            |_ctx: &mut ApplicationContext, event: InputEvent| {
                let scene = self_.clone().borrow_mut().current_scene.clone();
                scene.borrow_mut().handle_event(self_.clone(), event);

                if self_.borrow_mut().pending_scene_change {
                    if self_.borrow_mut().pending_scene_change_deep_refresh {
                        UiController::full_refresh(self_.clone());
                    } else {
                        UiController::partial_refresh(self_.clone());
                    }
                    self_.borrow_mut().pending_scene_change = false;
                }

                while needs_redraw() {
                    reset_redraw();
                    let scene = self_.clone().borrow_mut().current_scene.clone();
                    scene.borrow_mut().draw(self_.clone());
                }
            },
        );
    }
}

pub struct Scene<State: ?Sized> {
    components: Vec<Box<dyn UiComponent<State>>>,
    state: Box<State>,
}

pub trait SceneTrait {
    fn initialize(&mut self);
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
    fn initialize(&mut self) {
        for component in self.components.iter_mut() {
            component.initialize(&mut self.state);
        }
    }

    fn draw(&self, ui: Rc<RefCell<&mut UiController>>) {
        for component in &self.components {
            component.draw(ui.clone(), &self.state);
        }
    }

    fn handle_event(&mut self, ui: Rc<RefCell<&mut UiController>>, event: InputEvent) {
        for component in self.components.iter_mut() {
            component.handle_event(ui.clone(), &mut self.state, &event);
        }
    }
}

pub trait UiComponent<State: ?Sized> {
    fn initialize(&mut self, _state: &mut State) {}
    fn handle_event(
        &mut self,
        _ui: Rc<RefCell<&mut UiController>>,
        _state: &mut State,
        _event: &InputEvent,
    ) {
    }
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
