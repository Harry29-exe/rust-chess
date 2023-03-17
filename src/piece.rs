use std::fmt::{Display, Formatter};
use crate::board::{Board, BoardPosition};
use crate::piece::PieceColor::{Black, White};

#[derive(Copy, Clone, PartialEq)]
pub struct PieceState {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub moved: PieceMoved
}

// impl Display for PieceState {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self.piece_type {
//             PieceType::ROOK => f.pad("r"),
//             PieceType::KNIGHT => f.pad("k"),
//             PieceType::BISHOP => f.pad("b"),
//             PieceType::QUEEN => f.pad("q"),
//             PieceType::KING => f.pad("K"),
//             PieceType::Pawn => f.pad("p")
//         }
//     }
// }

impl Display for PieceState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color_str = match self.color {
            PieceColor::White => "\x1B[38;2;255;255;255m", // White color
            PieceColor::Black => "\x1B[38;2;0;0;0m", // Black color
        };
        let piece_str = match self.piece_type {
            PieceType::ROOK => "r",
            PieceType::KNIGHT => "k",
            PieceType::BISHOP => "b",
            PieceType::QUEEN => "q",
            PieceType::KING => "K",
            PieceType::Pawn => "p"
        };
        write!(f, "{}{}{}", color_str, piece_str, "\x1B[0m") // Reset color
    }
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