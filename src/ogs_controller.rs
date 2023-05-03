use crate::game_controller::{ControllerOption, GameController};
use crate::go::BoardState;
use crate::utility::vec_of_strings;
use cgmath::Point2;
use std::collections::HashMap;

pub struct OgsController {
    state: BoardState,
}

impl OgsController {
    pub(crate) fn new(options: HashMap<String, String>) -> OgsController {
        let board_size = options.get("Board Size").unwrap();
        let initial_state = BoardState::new(board_size.split("x").next().unwrap().parse().unwrap());
        OgsController {
            state: initial_state,
        }
    }
}

pub fn options() -> Vec<ControllerOption> {
    vec![
        ControllerOption::new("Board Size", vec!["9x9", "13x13", "19x19"]),
        ControllerOption::new("Clock", vec!["Blitz", "Rapid", "None"]),
    ]
}

impl GameController for OgsController {
    fn current_game_state(&self) -> &BoardState {
        &self.state
    }

    fn try_play(&mut self, point: Point2<usize>) -> Result<(), &str> {
        self.state.try_play(point)
    }
}
