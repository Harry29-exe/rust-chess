use crate::board::{Board, BOARD_WIDTH, BoardPosition};
use crate::moves::get_moves;

mod piece;
mod board;
mod moves;

fn main() {
    let mut board = Board::new();

    for row in 0..BOARD_WIDTH {
        for col in 0..BOARD_WIDTH {
            print!("{}",  board.value(col, row))
        }
        println!()
    }
    let moves = get_moves(&board, &BoardPosition{x: 1, y: 1});
    for pos in moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }
    println!();

    board.move_piece(&BoardPosition{x: 0, y: 6}, &BoardPosition{x: 0, y: 1});
    let moves = get_moves(&board, &BoardPosition{x: 1, y: 1});
    for pos in moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }

    println!("Hello, world!");
}
