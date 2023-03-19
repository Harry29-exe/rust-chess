use crate::board::{Board, Position};
use crate::moves::get_moves;

mod board;
mod moves;
mod errors;
#[cfg(test)]
mod moves_tests;

fn main() {
    let mut board = Board::new();
    let mut moves: Vec<Position> = Vec::new();

    println!("{}", board);
    get_moves(&board, &Position {x: 1, y: 1}, &mut moves);
    for pos in &moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }
    moves.clear();

    board.move_piece(&Position {x: 0, y: 6}, &Position {x: 0, y: 2});
    println!("\n{}", board);
    get_moves(&board, &Position {x: 1, y: 1}, &mut moves);
    for pos in &moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }
    moves.clear();


    board.move_piece(&Position {x: 0, y: 2}, &Position {x: 0, y: 1});
    println!("\n{}", board);
    get_moves(&board, &Position {x: 0, y: 0}, &mut moves);
    for pos in &moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }
    moves.clear();

}
