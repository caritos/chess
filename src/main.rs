
// using 2d array representation for simplicity
fn main() {
    let board = initialize_board();
    print_board(&board);

    println!("\nMoves for white pawn at (6, 4):");
    let pawn_moves = generate_pawn_moves(&board, 6, 4, true);
    for (r, c) in pawn_moves {
        println!("({},{})", r, c);
    }

    println!("\nMoves for black pawn at (1, 4):");
    let pawn_moves = generate_pawn_moves(&board, 1, 4, false);
    for (r, c) in pawn_moves {
        println!("({},{})", r, c);
    }

    println!("\nMoves for white rook at (7, 0):");
    let rook_moves = generate_rook_moves(&board, 7, 0, true);
    for (r, c) in rook_moves {
        println!("({},{})", r, c);
    }

    println!("\nMoves for black rook at (0, 0):");
    let rook_moves = generate_rook_moves(&board, 0, 0, false);
    for (r, c) in rook_moves {
        println!("({},{})", r, c);
    }
}


type Board = [[Option<Piece>; 8]; 8];

#[derive(Debug, Clone, Copy)] // allows us to print the pieces for debugging and enables easy
                              // copying of the pieces when needed, espeically for a board array
enum Piece {
    King(bool),   // true for white, false for black
    Queen(bool),
    Rook(bool),
    Bishop(bool),
    Knight(bool),
    Pawn(bool),
}

fn initialize_board() -> [[Option<Piece>; 8]; 8] {
    use Piece::*;
    [
        [Some(Rook(false)), Some(Knight(false)), Some(Bishop(false)), Some(Queen(false)), Some(King(false)), Some(Bishop(false)), Some(Knight(false)), Some(Rook(false))],
        [Some(Pawn(false)); 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [None; 8],
        [Some(Pawn(true)); 8],
        [Some(Rook(true)), Some(Knight(true)), Some(Bishop(true)), Some(Queen(true)), Some(King(true)), Some(Bishop(true)), Some(Knight(true)), Some(Rook(true))],
    ]
}

fn print_board(board: &Board) {
    for row in board.iter() {
        for cell in row.iter() {
            match cell {
                Some(piece) => print!("{:?} ", piece_to_symbol(piece)),
                None => print!(". "),
            }
        }
        println!();
    }
}

fn piece_to_symbol(piece: &Piece) -> char {
    match piece {
        Piece::King(true) => 'K',     // White King
        Piece::Queen(true) => 'Q',    // White Queen
        Piece::Rook(true) => 'R',     // White Rook
        Piece::Bishop(true) => 'B',   // White Bishop
        Piece::Knight(true) => 'N',   // White Knight
        Piece::Pawn(true) => 'P',     // White Pawn
        Piece::King(false) => 'k',    // Black King
        Piece::Queen(false) => 'q',   // Black Queen
        Piece::Rook(false) => 'r',    // Black Rook
        Piece::Bishop(false) => 'b',  // Black Bishop
        Piece::Knight(false) => 'n',  // Black Knight
        Piece::Pawn(false) => 'p',    // Black Pawn
    }
}

fn is_in_bounds(row: isize, col: isize) -> bool {
    row >= 0 && row < 8 && col >= 0 && col < 8
}

fn is_opponent(piece: Option<Piece>, is_white: bool) -> bool {
    match piece {
        Some(Piece::King(color)) |
        Some(Piece::Queen(color)) |
        Some(Piece::Rook(color)) |
        Some(Piece::Bishop(color)) |
        Some(Piece::Knight(color)) |
        Some(Piece::Pawn(color)) => color != is_white,
        None => false,
    }
}

fn generate_pawn_moves(board: &Board, row: usize, col: usize, is_white: bool) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let direction = if is_white { -1 } else { 1 }; // White moves up, black moves down

    // Forward move
    let next_row = row as isize + direction;
    if is_in_bounds(next_row, col as isize) && board[next_row as usize][col].is_none() {
        moves.push((next_row as usize, col));

        // Double move from starting position
        if (is_white && row == 6) || (!is_white && row == 1) {
            let double_row = next_row + direction;
            if is_in_bounds(double_row, col as isize) && board[double_row as usize][col].is_none() {
                moves.push((double_row as usize, col));
            }
        }
    }

    // Captures
    for &dc in [-1, 1].iter() {
        let capture_col = col as isize + dc;
        if is_in_bounds(next_row, capture_col) {
            let target = board[next_row as usize][capture_col as usize];
            if is_opponent(target, is_white) {
                moves.push((next_row as usize, capture_col as usize));
            }
        }
    }

    moves
}

       }
    }

    moves
}

fn generate_rook_moves(board: &Board, row: usize, col: usize, is_white: bool) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    // Directions: up, down, left, right
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for &(dr, dc) in directions.iter() {
        let mut next_row = row as isize + dr;
        let mut next_col = col as isize + dc;

        while is_in_bounds(next_row, next_col) {
            let target = board[next_row as usize][next_col as usize];

            if target.is_none() {
                // Empty square
                moves.push((next_row as usize, next_col as usize));
            } else if is_opponent(target, is_white) {
                // Capture opponent piece
                moves.push((next_row as usize, next_col as usize));
                break; // Stop further movement in this direction
            } else {
                // Blocked by same color piece
 break;
            }

            next_row += dr;
            next_col += dc;
 
