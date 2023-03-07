use crate::board::{Board, BoardPosition};
use crate::piece::{Moves, Piece, PieceColor, PieceMoved};
use crate::piece::Piece::{Blank, Pawn};
use crate::piece::PieceColor::{Black, White};

struct MoveService {}

impl MoveService {
    pub fn get_moves(board: &Board, pos: &BoardPosition) -> Moves {
        let piece = board.value_at(pos);
        return match piece {
            Piece::Pawn(color, moved) => MoveService::get_moves_pawn(board, color, moved, &pos),
            _ => panic!()
        };
    }

    #[inline]
    fn get_moves_pawn(board: &Board, piece_color: PieceColor, moved: PieceMoved, pos: &BoardPosition) -> Moves {
        let mut moves: Vec<BoardPosition> = Vec::new();
        let forward_y: isize = match piece_color {
            White => 1,
            Black => -1,
        };

        let pos_forward = pos.delta_y(forward_y);
        if board.value_at(&pos_forward) == Blank {
            moves.push(pos_forward);
            let pos_forward2 = pos.delta_y(forward_y+forward_y);
            if moved == PieceMoved::No && board.value_at(&pos_forward2) == Blank {
                moves.push(pos_forward2);
            }
        }


        if pos.x > 0 {
            let left = pos.delta_x(-1);
            let leftPiece = board.value_at(&left);
            match leftPiece {
                Pawn(left_color, left_moved) => {
                    if piece_color.opposite_to(&left_color) { }
                }
            }
        }

        moves
    }
}
