use crate::board::{Board, BOARD_WIDTH, BoardPosition};
use crate::moves::get_moves;

mod piece;
mod board;
mod moves;
mod errors;

fn main() {
    let mut board = Board::new();

    println!("{}", board);
    let moves = get_moves(&board, &BoardPosition{x: 1, y: 1});
    for pos in moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }

    board.move_piece(&BoardPosition{x: 0, y: 6}, &BoardPosition{x: 0, y: 2});
    // print_board(&board);
    println!("\n{}", board);
    let moves = get_moves(&board, &BoardPosition{x: 1, y: 1});
    for pos in moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }


    board.move_piece(&BoardPosition{x: 0, y: 2}, &BoardPosition{x: 0, y: 1});
    println!("\n{}", board);
    let moves = get_moves(&board, &BoardPosition{x: 0, y: 0});
    for pos in moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }

}
