use std::io;

const UNASSIGNED: u8 = 0;

fn print_board(board: &[[u8; 9]; 9]) {
    for row in 0..9 {
        for col in 0..9 {
            print!("{} ", board[row][col]);
        }
        println!();
    }
}

fn find_unassigned_location(board: &[[u8; 9]; 9], row: &mut usize, col: &mut usize) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if board[i][j] == UNASSIGNED {
                *row = i;
                *col = j;
                return true;
            }
        }
    }
    return false;
}

fn used_in_row(board: &[[u8; 9]; 9], row: usize, num: u8) -> bool {
    for col in 0..9 {
        if board[row][col] == num {
            return true;
        }
    }
    return false;
}

fn used_in_col(board: &[[u8; 9]; 9], col: usize, num: u8) -> bool {
    for row in 0..9 {
        if board[row][col] == num {
            return true;
        }
    }
    return false;
}

fn used_in_box(board: &[[u8; 9]; 9], box_start_row: usize, box_start_col: usize, num: u8) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if board[row + box_start_row][col + box_start_col] == num {
                return true;
            }
        }
    }
    return false;
}

fn is_safe(board: &[[u8; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    return !used_in_row(board, row, num) &&
           !used_in_col(board, col, num) &&
           !used_in_box(board, row - row % 3, col - col % 3, num) &&
           board[row][col] == UNASSIGNED;
}

fn solve_board(board: &mut [[u8; 9]; 9]) -> bool {
    let mut row = 0;
    let mut col = 0;

    if !find_unassigned_location(board, &mut row, &mut col) {
        return true;
    }

    for num in 1..10 {
        if is_safe(board, row, col, num) {
            board[row][col] = num;

            if solve_board(board) {
                return true;
            }

            board[row][col] = UNASSIGNED;
        }
    }

    return false;
}

fn main() {
    let mut board: [[u8; 9]; 9] = [[0; 9]; 9]; // 9 by 9 grilled
    
    // Read input
    for row in 0..9 {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => (),
            Err(error) => println!("error: {}", error),
        }

        let row_value: Vec<u8> = line.split_whitespace()
            .map(|s| s.parse::<u8>().unwrap_or(0))
            .collect();

        if row_value.len() != 9 {
            panic!("Invalid input");
        }

        board[row as usize] = row_value.try_into().unwrap();
    }

    println!("Input board:");
    print_board(&board);
    solve_board(&mut board);
    println!("Solved board:");
    print_board(&board);
}
