use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, PartialEq)]
pub struct PieceState {
    pub piece_type: Type,
    pub color: Color,
    pub moved: PieceMoved
}

impl Display for PieceState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color_str = match self.color {
            Color::White => "\x1B[38;2;255;255;255m", // White color
            Color::Black => "\x1B[38;2;0;0;0m", // Black color
        };
        let piece_str = match self.piece_type {
            Type::Rook => "r",
            Type::Knight => "k",
            Type::Bishop => "b",
            Type::Queen => "q",
            Type::KING => "K",
            Type::Pawn => "p"
        };
        write!(f, "{}{}{}", color_str, piece_str, "\x1B[0m") // Reset color
    }
}

impl PieceState {
    pub fn fmt_utf8(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let color_str= "\x1B[38;2;0;0;0m";
        let piece_str = match (self.piece_type, self.color) {
            (Type::KING, Color::White) => "♔",
            (Type::KING, Color::Black) => "♚",
            (Type::Queen, Color::White) => "♕",
            (Type::Queen, Color::Black) => "♛",
            (Type::Rook, Color::White) => "♖",
            (Type::Rook, Color::Black) => "♜",
            (Type::Bishop, Color::White) => "♗",
            (Type::Bishop, Color::Black) => "♝",
            (Type::Knight, Color::White) => "♘",
            (Type::Knight, Color::Black) => "♞",
            (Type::Pawn, Color::White) => "♙",
            (Type::Pawn, Color::Black) => "♟︎"
        };
        write!(f, "{}{}{}", color_str, piece_str, "\x1B[0m") // Reset color
    }
}


#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub enum Type {
    Rook,
    Knight,
    Bishop,
    Queen,
    KING,
    Pawn,
}

#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[inline]
    pub(crate) fn opposite(&self) -> Color {
        return match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
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
