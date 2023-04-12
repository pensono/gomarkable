use cgmath::Point2;

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
        }
    }

    pub fn play(self: &mut BoardState, point: Point2<usize>) {
        // Don't implement any fancy logic yet
        self.board[point.x][point.y] = Some(self.current_player);
        self.current_player = other_player(self.current_player);
        self.last_move = Some(point);
    }
}