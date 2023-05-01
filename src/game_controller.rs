use std::collections::HashMap;
use cgmath::Point2;
use crate::go::BoardState;

pub struct ControllerOption {
    pub(crate) name: String,
    pub(crate) values: Vec<String>,
}

pub trait GameController {
    fn current_game_state(&self) -> &BoardState;
    fn try_play(&mut self, point: Point2<usize>) -> Result<(), &str>;
}

impl ControllerOption {
    pub fn new(name: &str, values: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            values: values.into_iter().map(|x| x.to_string()).collect(),
        }
    }
}
