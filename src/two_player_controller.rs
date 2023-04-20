use cgmath::Point2;
use crate::game_controller::GameController;
use crate::go::BoardState;


pub struct TwoPlayerController {
    state: BoardState,
}

impl TwoPlayerController {
    pub(crate) fn new(initial_state: BoardState) -> TwoPlayerController {
        TwoPlayerController {
            state: initial_state,
        }
    }
}

impl GameController for TwoPlayerController {
    fn current_game_state(&self) -> &BoardState {
        &self.state
    }

    fn try_play(&mut self, point: Point2<usize>) -> Result<(), &str> {
        self.state.try_play(point)
    }
}
