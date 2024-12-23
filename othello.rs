
const SIZE: usize = 8;
const EMPTY: char = '.';
const BLACK: char = 'B';
const WHITE: char = 'W';

type Board = [[char; SIZE]; SIZE];

fn initialize_board() -> Board {
    let mut board = [[EMPTY; SIZE]; SIZE];
    board[SIZE / 2 - 1][SIZE / 2 - 1] = WHITE;
    board[SIZE / 2][SIZE / 2] = WHITE;
    board[SIZE / 2 - 1][SIZE / 2] = BLACK;
    board[SIZE / 2][SIZE / 2 - 1] = BLACK;
    board
}

fn display_board(board: &Board) {
    for row in board.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn is_valid_move(board: &Board, row: usize, col: usize, player: char) -> bool {
    if board[row][col] != EMPTY {
        return false;
    }
    let opponent = if player == BLACK { WHITE } else { BLACK };
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for &(dr, dc) in &directions {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        let mut found_opponent = false;
        while r >= 0 && r < SIZE as isize && c >= 0 && c < SIZE as isize {
            match board[r as usize][c as usize] {
                cell if cell == opponent => found_opponent = true,
                cell if cell == player && found_opponent => return true,
                _ => break,
            }
            r += dr;
            c += dc;
        }
    }
    false
}

fn apply_move(board: &mut Board, row: usize, col: usize, player: char) {
    board[row][col] = player;
    let opponent = if player == BLACK { WHITE } else { BLACK };
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    for &(dr, dc) in &directions {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        let mut flips = vec![];
        while r >= 0 && r < SIZE as isize && c >= 0 && c < SIZE as isize {
            match board[r as usize][c as usize] {
                cell if cell == opponent => flips.push((r as usize, c as usize)),
                cell if cell == player => {
                    for &(fr, fc) in &flips {
                        board[fr][fc] = player;
                    }
                    break;
                }
                _ => break,
            }
            r += dr;
            c += dc;
        }
    }
}

fn find_moves(board: &Board, player: char) -> Vec<(usize, usize)> {
    let mut moves = vec![];
    for row in 0..SIZE {
        for col in 0..SIZE {
            if is_valid_move(board, row, col, player) {
                moves.push((row, col));
            }
        }
    }
    moves
}

fn count_pieces(board: &Board, player: char) -> usize {
    board.iter().flatten().filter(|&&cell| cell == player).count()
}

fn main() {
    let mut board = initialize_board();
    let mut current_player = BLACK;

    loop {
        display_board(&board);
        let moves = find_moves(&board, current_player);
        if moves.is_empty() {
            println!("No valid moves for {}!", current_player);
            current_player = if current_player == BLACK { WHITE } else { BLACK };
            if find_moves(&board, current_player).is_empty() {
                println!("Game over!");
                let black_count = count_pieces(&board, BLACK);
                let white_count = count_pieces(&board, WHITE);
                println!("Black: {}, White: {}", black_count, white_count);
                if black_count > white_count {
                    println!("Black wins!");
                } else if white_count > black_count {
                    println!("White wins!");
                } else {
                    println!("It's a draw!");
                }
                break;
            }
            continue;
        }

        let (row, col) = moves[0];
        println!("{} plays at ({}, {})", current_player, row, col);
        apply_move(&mut board, row, col, current_player);

        current_player = if current_player == BLACK { WHITE } else { BLACK };
    }
}
