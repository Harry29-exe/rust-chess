use crate::board::{Board, BOARD_WIDTH, BoardField, BoardPosition};
use crate::piece::{PieceMoved, PieceState};
use crate::piece::PieceType::{Pawn};
use crate::piece::PieceColor::{Black, White};



pub fn get_moves(board: &Board, pos: &BoardPosition) -> Vec<BoardPosition> {
    let piece = board.value_at(pos);
    return match piece {
        BoardField::Empty => panic!("Empty fields don't have any moves"),
        BoardField::Piece(state) => {
            match state.piece_type {
                Pawn => get_moves_pawn(board, &state, pos),
                _ => panic!("not implemented")
            }
        }
    };
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
    if board.empty(&pos_forward) {
        moves.push(pos_forward);
        let pos_forward2 = piece_pos.delta_y(forward_y+forward_y);
        if piece_state.moved == PieceMoved::No && board.empty(&pos_forward2) {
            moves.push(pos_forward2);
        }
    }


    if piece_pos.x > 0 {
        let left = piece_pos.delta(-1, forward_y);
        if board.empty_or_color(&left, &opposite_color) {
            moves.push(left)
        }
    }

    if piece_pos.x < BOARD_WIDTH - 1 {
        let right = piece_pos.delta(1, forward_y);
        if board.empty_or_color(&right, &opposite_color) {
            moves.push(right)
        }
    }

    moves
}
