use crate::game_controller::{ControllerOption, GameController};
use crate::go::BoardState;
use cgmath::Point2;
use std::collections::HashMap;

pub struct OnePlayerController {
    state: BoardState,
}

impl OnePlayerController {
    pub(crate) fn new(options: HashMap<String, String>) -> OnePlayerController {
        let board_size = options.get("Board Size").unwrap();
        let initial_state = BoardState::new(board_size.split("x").next().unwrap().parse().unwrap());
        OnePlayerController {
            state: initial_state,
        }
    }
}

pub fn options() -> Vec<ControllerOption> {
    vec![
        ControllerOption::new("Board Size", vec!["9x9", "13x13", "19x19"]),
        ControllerOption::new("Difficulty", vec!["Easy", "Medium", "Hard"]),
        ControllerOption::new("Handicap", vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]),
    ]
}

impl GameController for OnePlayerController {
    fn current_game_state(&self) -> &BoardState {
        &self.state
    }

    fn try_play(&mut self, point: Point2<usize>) -> Result<(), &str> {
        self.state.try_play(point)
    }
}
