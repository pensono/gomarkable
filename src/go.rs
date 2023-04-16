use std::collections::HashSet;
use cgmath::{Point2, point2};

// An enum for each player
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Black,
    White,
}

fn other_player(player: Player) -> Player {
    match player {
        Player::Black => Player::White,
        Player::White => Player::Black,
    }
}

// A struct representing the state of a Go Board
pub struct BoardState {
    // The size of the board
    pub size: usize,
    // The current player
    pub current_player: Player,
    // The current board state
    pub board: Vec<Vec<Option<Player>>>,
    pub last_move: Option<Point2<usize>>,
    pub ko: Option<Point2<usize>>,

    pub captured_black: u32,
    pub captured_white: u32,
    pub komi_minus_half: u32,
}

impl BoardState {
    pub fn new(size: usize) -> BoardState {
        let mut board = Vec::new();
        for _ in 0..size {
            let mut row = Vec::new();
            for _ in 0..size {
                row.push(None);
            }
            board.push(row);
        }

        BoardState {
            size,
            current_player: Player::Black,
            board,
            last_move: None,
            ko: None,
            captured_black: 0,
            captured_white: 0,
            komi_minus_half: 6,
        }
    }

    pub fn captures_made_by(&self, player: Player) -> u32 {
        match player {
            Player::Black => self.captured_black,
            Player::White => self.captured_white,
        }
    }

    pub fn try_play(self: &mut BoardState, point: Point2<usize>) -> Result<(), &str> {
        // Can't play where a piece already is
        if self.board[point.x][point.y].is_some() {
            return Err("Can't play where a piece already is");
        }

        // Ko rule
        if let Some(ko) = self.ko {
            if point == ko {
                return Err("Can't play in the ko");
            }
        }

        self.board[point.x][point.y] = Some(self.current_player);

        // Capture other pieces
        for neighbor in self.get_neighbours(&point) {
            if self.board[neighbor.x][neighbor.y] != Some(other_player(self.current_player)) {
                continue;
            }

            let line = self.get_line(&neighbor);
            let liberties = self.get_liberties(&line);
            if liberties.len() == 0 {
                for point in line {
                    self.board[point.x][point.y] = None;
                    if self.current_player == Player::Black {
                        self.captured_black += 1;
                    } else {
                        self.captured_white += 1;
                    }
                }
            }
        }

        // If the current player does not have any liberties, return false
        // Note that this will never be true if any pieces were just captured
        let played_line = self.get_line(&point);
        let liberties = self.get_liberties(&played_line);
        if liberties.len() == 0 {
            self.board[point.x][point.y] = None;
            return Err("Self capture");
        }

        self.current_player = other_player(self.current_player);
        self.last_move = Some(point);

        return Ok(());
    }

    pub fn get_line(self: &mut BoardState, point: &Point2<usize>) -> Vec<Point2<usize>> {
        let color = self.board[point.x][point.y];
        if color == None {
            return vec![];
        }

        let mut to_visit = vec![];
        let mut visited = HashSet::new();
        let mut line = vec![];

        to_visit.push(point.clone());

        while to_visit.len() > 0 {
            let current = to_visit.pop().unwrap();
            visited.insert(current);

            if !(self.board[current.x][current.y] == color) {
                continue;
            }

            line.push(current);
            let neighbors = self.get_neighbours(&current);
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    to_visit.push(neighbor.clone());
                }
            }
        }

        return line;
    }

    pub fn get_liberties(self: &mut BoardState, line: &Vec<Point2<usize>>) -> Vec<Point2<usize>> {
        let mut liberties = HashSet::new();

        for point in line {
            let neighbors = self.get_neighbours(&point);
            for neighbor in neighbors {
                if self.board[neighbor.x][neighbor.y].is_none() {
                    liberties.insert(neighbor);
                }
            }
        }

        return liberties.into_iter().collect();
    }

    pub fn get_neighbours(self: &mut BoardState, point: &Point2<usize>) -> Vec<Point2<usize>> {
        let mut neighbours = vec![];

        if point.x > 0 {
            neighbours.push(point2(point.x - 1, point.y));
        }
        if point.x < self.size - 1 {
            neighbours.push(point2(point.x + 1, point.y));
        }
        if point.y > 0 {
            neighbours.push(point2(point.x, point.y - 1));
        }
        if point.y < self.size - 1 {
            neighbours.push(point2(point.x, point.y + 1));
        }

        return neighbours;
    }
}

#[cfg(test)]
mod tests {
    use cgmath::point2;

    #[test]
    fn onlyPlayEachOnce() {
        let mut state = super::BoardState::new(19);
        assert_eq!(state.try_play(point2(10, 10)), true);
        assert_eq!(state.try_play(point2(10, 10)), false);
    }

    #[test]
    fn ko() {
        let mut state = super::BoardState::new(19);
        state.ko = Some(point2(10, 10));
        assert_eq!(state.try_play(point2(10, 10)), false);
    }

    #[test]
    fn capture() {
        let mut state = super::BoardState::new(19);

        state.board[9][10] = Some(super::Player::Black);
        state.board[10][9] = Some(super::Player::Black);
        state.board[11][10] = Some(super::Player::Black);
        state.board[10][10] = Some(super::Player::White);

        assert_eq!(state.try_play(point2(10, 11)), true);

        assert_eq!(state.captured_white, 1);
        assert_eq!(state.captured_black, 0);
        assert_eq!(state..board[10][10], None);
    }
}