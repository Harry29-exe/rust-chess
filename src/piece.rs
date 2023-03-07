use crate::Board;
use crate::board::BoardPosition;
use crate::piece::Piece::Blank;
use crate::piece::PieceColor::{Black, White};

#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    ROOK(PieceColor, PieceMoved),
    KNIGHT(PieceColor),
    BISHOP(PieceColor),
    QUEEN(PieceColor),
    KING(PieceColor, PieceMoved),
    Pawn(PieceColor, PieceMoved),
    Blank,
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    #[inline]
    pub(crate) fn opposite(&self) -> PieceColor {
        return match self {
            White => Black,
            Black => White,
        }
    }

    #[inline]
    pub(crate) fn opposite_to(&self, color: &PieceColor) -> bool {
        return self != color
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum PieceMoved {
    Yes,
    LastTurn,
    No,
}

pub type Moves = Vec<BoardPosition>;