use std::fmt::Formatter;
use crate::board::{Board, BoardPosition};
use crate::piece::PieceColor::{Black, White};

#[derive(Copy, Clone, PartialEq)]
pub struct PieceState {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub moved: PieceMoved
}

#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub enum PieceType {
    ROOK,
    KNIGHT,
    BISHOP,
    QUEEN,
    KING,
    Pawn,
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceType::ROOK => f.pad("r"),
            PieceType::KNIGHT => f.pad("k"),
            PieceType::BISHOP => f.pad("b"),
            PieceType::QUEEN => f.pad("q"),
            PieceType::KING => f.pad("K"),
            PieceType::Pawn => f.pad("p")
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
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
}

#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub enum PieceMoved {
    Yes,
    LastTurn,
    No,
}

pub type Moves = Vec<BoardPosition>;