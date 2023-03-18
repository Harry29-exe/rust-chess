use crate::board::{Board, BOARD_WIDTH, BoardPosition};
use crate::moves::get_moves;

mod piece;
mod board;
mod moves;
mod errors;

fn main() {
    let mut board = Board::new();
    let mut moves: Vec<BoardPosition> = Vec::new();

    println!("{}", board);
    get_moves(&board, &BoardPosition{x: 1, y: 1}, &mut moves);
    for pos in &moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }
    moves.clear();

    board.move_piece(&BoardPosition{x: 0, y: 6}, &BoardPosition{x: 0, y: 2});
    println!("\n{}", board);
    get_moves(&board, &BoardPosition{x: 1, y: 1}, &mut moves);
    for pos in &moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }
    moves.clear();


    board.move_piece(&BoardPosition{x: 0, y: 2}, &BoardPosition{x: 0, y: 1});
    println!("\n{}", board);
    get_moves(&board, &BoardPosition{x: 0, y: 0}, &mut moves);
    for pos in &moves {
        println!("x: {}, y: {}", pos.x, pos.y);
    }
    moves.clear();

}
