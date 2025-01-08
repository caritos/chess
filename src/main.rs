#[derive(Debug, Clone, Copy)]
enum Piece {
    King(bool),    // true = white, false = black
    Queen(bool),
    Rook(bool),
    Bishop(bool),
    Knight(bool),
    Pawn(bool),
}

type Board = [[Option<Piece>; 8]; 8];

fn initialize_board() -> Board {
    let mut board = [[None; 8]; 8];

    // Place pawns
    for col in 0..8 {
        board[1][col] = Some(Piece::Pawn(false)); // Black pawns
        board[6][col] = Some(Piece::Pawn(true));  // White pawns
    }

    // Place rooks
    board[0][0] = Some(Piece::Rook(false));
    board[0][7] = Some(Piece::Rook(false));
    board[7][0] = Some(Piece::Rook(true));
    board[7][7] = Some(Piece::Rook(true));

    // Place knights
    board[0][1] = Some(Piece::Knight(false));
    board[0][6] = Some(Piece::Knight(false));
    board[7][1] = Some(Piece::Knight(true));
    board[7][6] = Some(Piece::Knight(true));

    // Place bishops
    board[0][2] = Some(Piece::Bishop(false));
    board[0][5] = Some(Piece::Bishop(false));
    board[7][2] = Some(Piece::Bishop(true));
    board[7][5] = Some(Piece::Bishop(true));

    // Place queens
    board[0][3] = Some(Piece::Queen(false));
    board[7][3] = Some(Piece::Queen(true));

    // Place kings
    board[0][4] = Some(Piece::King(false));
    board[7][4] = Some(Piece::King(true));

    board
}

fn print_board(board: &Board) {
    for row in board.iter() {
        for cell in row.iter() {
            match cell {
                Some(piece) => print!("{} ", piece_to_symbol(piece)),
                None => print!(". "), // Empty squares
            }
        }
        println!(); // Newline at the end of each row
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
        }
    }

    moves
}

fn generate_bishop_moves(board: &Board, row: usize, col: usize, is_white: bool) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();

    // Directions: top-left, top-right, bottom-left, bottom-right
    let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

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
        }
    }

    moves
}


fn main() {
    let mut board = initialize_board();
    print_board(&board);

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

    // Test with a modified board (remove pawns)
    board[6][0] = None; // Remove white pawn in front of white rook
    board[1][0] = None; // Remove black pawn in front of black rook

    println!("\nModified board:");
    print_board(&board);

    println!("\nMoves for white rook at (7, 0) after modification:");
    let rook_moves = generate_rook_moves(&board, 7, 0, true);
    for (r, c) in rook_moves {
        println!("({},{})", r, c);
    }

    println!("\nMoves for white bishop at (7, 2):");
let bishop_moves = generate_bishop_moves(&board, 7, 2, true);
for (r, c) in bishop_moves {
    println!("({},{})", r, c);
}

println!("\nMoves for black bishop at (0, 2):");
let bishop_moves = generate_bishop_moves(&board, 0, 2, false);
for (r, c) in bishop_moves {
    println!("({},{})", r, c);
}

// Test with a modified board (remove pawns)
board[6][1] = None; // Remove white pawn blocking the white bishop
board[6][3] = None; // Remove white pawn blocking the white bishop

println!("\nModified board:");
print_board(&board);

println!("\nMoves for white bishop at (7, 2) after modification:");
let bishop_moves = generate_bishop_moves(&board, 7, 2, true);
for (r, c) in bishop_moves {
    println!("({},{})", r, c);
}
}

