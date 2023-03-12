use crate::board::{Board, BOARD_WIDTH, BoardPosition};
use crate::moves::get_moves;

mod piece;
mod board;
mod moves;
mod errors;

fn main() {
    let mut board = Board::new();

    print_board(&board);
    let moves = get_moves(&board, &BoardPosition{x: 1, y: 1});
    for pos in moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }

    println!();
    board.move_piece(&BoardPosition{x: 0, y: 6}, &BoardPosition{x: 0, y: 2});
    print_board(&board);
    let value = board.value(0, 2);
    println!("{}", value);

    let moves = get_moves(&board, &BoardPosition{x: 1, y: 1});
    for pos in moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }

    println!("Hello, world!");
}

fn print_board(board: &Board) {
    for row in 0..BOARD_WIDTH {
        for col in 0..BOARD_WIDTH {
            print!("{}", board.value(col, row))
        }
        println!()
    }
}
