use crate::board::{Board, BOARD_WIDTH};

mod piece;
mod board;
mod moves;

fn main() {
    let board = Board::new();

    for row in 0..BOARD_WIDTH {
        for col in 0..BOARD_WIDTH {
            print!("{}",  board.value(col, row))
        }
        println!()
    }
    println!("Hello, world!");
}
