use crate::game_controller::{ControllerOption, GameController};
use crate::go::Player;
use crate::ui::UiController;
use crate::utility::vec_of_strings;
use libremarkable::appctx::ApplicationContext;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod board_ui;
mod cgmath_extensions;
mod drawing;
mod game_controller;
mod go;
mod ogs_controller;
mod one_player_controller;
mod option_ui;
mod player_ui;
mod quit_ui;
mod text;
mod two_player_controller;
mod ui;
mod utility;

fn main() {
    let ctx = ApplicationContext::default();

    let mut initial_settings : HashMap<String, String> = HashMap::new();
    initial_settings.insert("Mode".to_string(), "2-Player".to_string());
    initial_settings.insert("Board Size".to_string(), "19x19".to_string());
    initial_settings.insert("Difficulty".to_string(), "Medium".to_string());
    initial_settings.insert("Handicap".to_string(), "0".to_string());
    initial_settings.insert("Clock".to_string(), "Rapid".to_string());
    initial_settings.insert("".to_string(), "Play".to_string()); // Dummy for play

    let menu = create_menu_scene(
        &ctx,
        two_player_controller::options(),
        initial_settings,
    );
    let mut controller = UiController::new(ctx, Rc::from(RefCell::new(menu)));
    let ui = Rc::from(RefCell::new(&mut controller));
    UiController::start(ui);
}

fn create_menu_scene(
    ctx: &ApplicationContext,
    options: Vec<ControllerOption>,
    state: HashMap<String, String>
) -> ui::Scene<HashMap<String, String>> {
    let mut menu = ui::Scene::new(state);

    menu.add(option_ui::OptionUi::new(
        ctx,
        400i32,
        "Mode".to_string(),
        vec_of_strings!["1-Player", "2-Player", "OGS"],
        Box::new(
            |ui: Rc<RefCell<&mut UiController>>,
             _state: &mut HashMap<String, String>,
             value: &String| {
                let options = controller_options_from_name(&*value);
                let scene = create_menu_scene(&ui.borrow_mut().context, options, _state.clone());
                UiController::change_scene(ui, Rc::from(RefCell::new(scene)), false);
            },
        ),
    ));

    let mut position = 600i32;
    for option in options {
        menu.add(option_ui::OptionUi::new(
            ctx,
            position,
            option.name,
            option.values,
            Box::new(
                move |_ui: Rc<RefCell<&mut UiController>>,
                      state: &mut HashMap<String, String>,
                      value: &String| {
                },
            ),
        ));

        position += 200;
    }

    menu.add(option_ui::OptionUi::new(
        ctx,
        1400i32,
        "".to_string(),
        vec_of_strings!["Play"],
        Box::new(
            |ui: Rc<RefCell<&mut UiController>>,
             state: &mut HashMap<String, String>,
             _value: &String| {
                let game_controller =
                    controller_from_name(&*state.get("Mode").unwrap(), state.clone());
                let scene = create_game_scene(&ui.borrow_mut().context, game_controller);
                UiController::change_scene(ui.clone(), Rc::from(RefCell::new(scene)), true);
            },
        ),
    ));

    return menu;
}

fn create_game_scene(
    ctx: &ApplicationContext,
    game_controller: Box<dyn GameController>,
) -> ui::Scene<Box<dyn GameController>> {
    let size = game_controller.current_game_state().size;
    let mut gameplay = ui::Scene::new(game_controller);
    gameplay.add(board_ui::BoardUi::new(ctx, size));
    gameplay.add(player_ui::PlayerUi::new(ctx, "White", true, Player::White));
    gameplay.add(player_ui::PlayerUi::new(ctx, "Black", false, Player::Black));
    gameplay.add(quit_ui::QuitUi::new(ctx));

    return gameplay;
}

fn controller_from_name(name: &str, options: HashMap<String, String>) -> Box<dyn GameController> {
    match name {
        "1-Player" => Box::new(one_player_controller::OnePlayerController::new(options)),
        "2-Player" => Box::new(two_player_controller::TwoPlayerController::new(options)),
        "OGS" => Box::new(ogs_controller::OgsController::new(options)),
        _ => panic!("Unknown game type"),
    }
}

fn controller_options_from_name(name: &str) -> Vec<ControllerOption> {
    match name {
        "1-Player" => one_player_controller::options(),
        "2-Player" => two_player_controller::options(),
        "OGS" => ogs_controller::options(),
        _ => panic!("Unknown game type"),
    }
}
