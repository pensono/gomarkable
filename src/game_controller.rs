use cgmath::Point2;
use crate::go::BoardState;

pub(crate) trait GameController {
    fn current_game_state(&self) -> &BoardState;
    fn try_play(&mut self, point: Point2<usize>) -> Result<(), &str>;
}
