use crate::{moves};
use crate::board::{Board, Field, Position};
use crate::board::piece::Color::{Black, White};
use crate::board::piece::{PieceMoved, PieceState};
use crate::board::piece::Type::Pawn;

#[test]
fn test_pawn_moves() {
    let tested_pawn_pos = Position{x: 1, y: 1};
    let board = Board::new_from(vec![
        (Field::new_white(Pawn), tested_pawn_pos.clone()),
        (Field::new_black(Pawn), Position{x: 0, y: 2}),
        (Field::new_black(Pawn), Position{x: 2, y: 2}),
    ]);

    let mut moves = Vec::new();
    moves::get_moves(&board, &tested_pawn_pos, &mut moves);
    assert_moves(&vec![
        Position{x: 0, y: 2},
        Position{x: 1, y: 2},
        Position{x: 1, y: 3},
        Position{x: 2, y: 2},
    ], &moves
    );
}

#[test]
fn test_pawn_moves_2() {
    let tested_pawn_pos = Position{x: 1, y: 2};
    let enemy_left_top = Position{x: 0, y: 3};
    let enemy_right_top = Position{x: 0, y: 3};
    let board = Board::new_from(vec![
        (Field::Piece(PieceState{piece_type: Pawn, color: White, moved: PieceMoved::Yes }), tested_pawn_pos.clone()),
        (Field::new_black(Pawn), enemy_left_top.clone()),
        (Field::new_black(Pawn), enemy_right_top.clone()),
    ]);

    let mut moves = Vec::new();
    moves::get_moves(&board, &tested_pawn_pos, &mut moves);
    let expected = vec![
        tested_pawn_pos.delta_y(1),
        enemy_left_top,
        enemy_right_top
    ];
    assert_moves(&expected, &moves);
}

fn assert_moves(expected: &Vec<Position>, actual: &Vec<Position>) {
    for expected_pos in expected {
        assert!(actual.iter().find(|pos| &expected_pos == pos).is_some());
    }
}
