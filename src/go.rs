// An enum for each player
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Black,
    White,
}

// A struct representing the state of a Go Board
pub struct BoardState {
    // The size of the board
    pub size: usize,
    // The current player
    pub current_player: Player,
    // The current board state
    pub board: Vec<Vec<Option<Player>>>,

    pub last_move: Option<(usize, usize)>,
    
    pub ko: Option<(usize, usize)>,
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
}