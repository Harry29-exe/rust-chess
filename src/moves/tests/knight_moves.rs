use crate::board::{Board, Field, Position};
use crate::board::piece::Color::White;
use crate::board::piece::{PieceMoved, PieceState};
use crate::board::piece::Type::{Knight, Pawn};
use crate::moves;
use crate::moves::tests::utils;

#[test]
fn test_knight_moves() {
    let tested_knight_pos = Position {x: 1, y: 1};
    let board = Board::new_from(vec![
        (Field::new_white(Knight), tested_knight_pos.clone()),
    ]);

    let mut moves = Vec::new();
    moves::get_moves(&board, &tested_knight_pos, &mut moves);
    utils::assert_moves(&vec![
        Position {x: 0, y: 3},
        Position {x: 2, y: 3},
        Position {x: 3, y: 2},
        Position {x: 3, y: 0},
    ], &moves);
}

#[test]
fn test_knight_moves_2() {
    let tested_knight_pos = Position {x: 4, y: 4};
    let board = Board::new_from(vec![
        (Field::new_white(Knight), tested_knight_pos.clone()),
    ]);

    let mut moves = Vec::new();
    moves::get_moves(&board, &tested_knight_pos, &mut moves);
    utils::assert_moves(&vec![
        Position {x: 6, y: 5},
        Position {x: 6, y: 3},
        Position {x: 5, y: 6},
        Position {x: 5, y: 2},
        Position {x: 3, y: 6},
        Position {x: 3, y: 2},
        Position {x: 2, y: 5},
        Position {x: 2, y: 3},
    ], &moves);
}

#[test]
fn test_knight_moves_3() {
    let tested_knight_pos = Position {x: 4, y: 4};
    let board = Board::new_from(vec![
        (Field::new_white(Knight), tested_knight_pos.clone()),
        (Field::new_white(Pawn), Position{x: 6, y: 5}),
        (Field::new_white(Pawn), Position{x: 2, y: 3}),
        (Field::new_black(Pawn), Position{x: 6, y: 3})
    ]);

    let mut moves = Vec::new();
    moves::get_moves(&board, &tested_knight_pos, &mut moves);
    utils::assert_moves(&vec![
        Position {x: 6, y: 3},
        Position {x: 5, y: 6},
        Position {x: 5, y: 2},
        Position {x: 3, y: 6},
        Position {x: 3, y: 2},
        Position {x: 2, y: 5},
    ], &moves);
}

