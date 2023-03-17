use std::io::empty;
use crate::board::{Board, BOARD_WIDTH, BoardField, BoardPosition};
use crate::piece::{PieceColor, PieceMoved, PieceState, PieceType};
use crate::piece::PieceColor::{Black, White};

pub fn get_moves(board: &Board, pos: &BoardPosition) -> Vec<BoardPosition> {
    let piece = board.value_at(pos);
    return match piece {
        BoardField::Empty => vec![],
        BoardField::Piece(state) => get_piece_moves(board, pos, &state)
    };
}

struct PossibleMovesService<'s> {
    vec: Vec<BoardPosition>,
    board: &'s Board,
    piece_pos: &'s BoardPosition,
    piece_state: &'s PieceState
}

#[inline]
fn get_piece_moves(board: &Board, pos: &BoardPosition, state: &PieceState) -> Vec<BoardPosition> {
    match state.piece_type {
        PieceType::Pawn => get_moves_pawn(board, &state, pos),
        PieceType::KNIGHT => get_moves_knight(board, &state, pos),
        PieceType::BISHOP => get_moves_bishop(board, &state, pos),
        PieceType::QUEEN => get_moves_queen(board, &state, pos),
        _ => panic!("not implemented")
    }
}

#[inline]
fn get_moves_pawn(board: &Board, piece_state: &PieceState, piece_pos: &BoardPosition) -> Vec<BoardPosition> {
    let mut moves: Vec<BoardPosition> = Vec::new();
    let opposite_color = piece_state.color.opposite();
    let forward_y: isize = match piece_state.color {
        White => Board::WHITE_FORWARD,
        Black => Board::BLACK_FORWARD,
    };

    let pos_forward = piece_pos.delta_y(forward_y);
    if board.is_empty(&pos_forward) {
        moves.push(pos_forward);
        let pos_forward2 = piece_pos.delta_y(forward_y+forward_y);
        if piece_state.moved == PieceMoved::No && board.is_empty(&pos_forward2) {
            moves.push(pos_forward2);
        }
    }


    if piece_pos.x > 0 {
        let left = piece_pos.delta(-1, forward_y);
        if board.is_color(&left, &opposite_color) {
            moves.push(left)
        }
    }

    if piece_pos.x < BOARD_WIDTH - 1 {
        let right = piece_pos.delta(1, forward_y);
        if board.is_color(&right, &opposite_color) {
            moves.push(right)
        }
    }

    moves
}

#[inline]
fn get_moves_knight(board: &Board, piece_state: &PieceState, piece_pos: &BoardPosition) -> Vec<BoardPosition> {
    let mut moves = Vec::new();
    get_moves_horizontal(board, &piece_state.color, &piece_pos, &mut moves);
    get_moves_vertical(board, &piece_state.color, &piece_pos, &mut moves);

    moves
}

#[inline]
fn get_moves_bishop(board: &Board, piece_state: &PieceState, piece_pos: &BoardPosition) -> Vec<BoardPosition> {
    let mut moves = Vec::new();
    get_moves_horizontal(board, &piece_state.color, piece_pos, &mut moves);

    moves
}

#[inline]
fn get_moves_queen(board: &Board, piece_state: &PieceState, piece_pos: &BoardPosition) -> Vec<BoardPosition> {
    let mut moves = Vec::new();

}

#[inline]
fn get_moves_vertical(board: &Board, piece_color: &PieceColor, piece_pos: &BoardPosition, moves: &mut Vec<BoardPosition>) {
    get_moves_loop(board, piece_color, piece_pos, 0, 1, moves);
    get_moves_loop(board, piece_color, piece_pos, 0, -1, moves);
}

#[inline]
fn get_moves_horizontal(board: &Board, piece_color: &PieceColor, piece_pos: &BoardPosition, moves: &mut Vec<BoardPosition>) {
    get_moves_loop(board, piece_color, piece_pos, 1, 0, moves);
    get_moves_loop(board, piece_color, piece_pos, -1, 0, moves);
}

#[inline]
fn get_moves_diagonal(board: &Board, piece_color: &PieceColor, piece_pos: &BoardPosition, moves: &mut Vec<BoardPosition>) {
    get_moves_loop(board, piece_color, piece_pos, -1, -1, moves);
    get_moves_loop(board, piece_color, piece_pos, -1, 1, moves);
    get_moves_loop(board, piece_color, piece_pos, 1, -1, moves);
    get_moves_loop(board, piece_color, piece_pos, 1, 1, moves);
}

#[inline]
fn get_moves_loop(board: &Board,
                  piece_color: &PieceColor,
                  piece_pos: &BoardPosition,
                  delta_x: isize,
                  delta_y: isize,
                  moves: &mut Vec<BoardPosition>
) {
    let opposite_color = piece_color.opposite();
    let mut previous_pos = piece_pos.clone();

    loop {
        let new_pos_result = previous_pos.delta_if_valid(delta_x, delta_y);
        if new_pos_result.is_err() {
            break;
        }

        let new_pos = new_pos_result.unwrap();
        if board.is_empty(&new_pos) {
            moves.push(new_pos.clone());
            previous_pos = new_pos;
            continue;
        }

        if board.is_color(&new_pos, &opposite_color) {
            moves.push(new_pos);
        }
        break;
    }
}
