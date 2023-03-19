use std::{fmt};
use std::fmt::{Formatter};
use crate::errors::ErrorKind;
use crate::board::piece::{Type, PieceState, PieceMoved, Color};
use crate::board::piece::Type::{BISHOP, KING, KNIGHT, Pawn, QUEEN, ROOK};

pub mod piece;
pub mod board_display;

pub const BOARD_WIDTH: usize = 8;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;


pub struct Board {
    board: [Field; BOARD_SIZE]
}

impl Board {
    pub const WHITE_START_ROW: usize = 0;
    const WHITE_PAWN_ROW: usize = 1;
    pub const WHITE_FORWARD: isize = 1;
    pub const BLACK_START_ROW: usize = 7;
    const BLACK_PAWN_ROW: usize = 6;
    pub const BLACK_FORWARD: isize = -1;

    #[inline]
    pub(crate) fn value_at(&self, pos: &Position) -> &Field {
        return &self.board[pos.y*BOARD_WIDTH + pos.x]
    }
    
    #[inline]
    pub(crate) fn is_color(&self, pos: &Position, color: &Color) -> bool {
        return match self.value_at(pos) {
            Field::Empty => false,
            Field::Piece(state) => state.color == *color
        }
    }

    #[inline]
    pub(crate) fn is_empty(&self, pos: &Position) -> bool {
        return self.value_at(pos) == &Field::Empty
    }

    #[inline]
    pub(crate) fn is_empty_or_color(&self, pos: &Position, color: &Color) -> bool {
        return match self.value_at(pos) {
            Field::Empty => true,
            Field::Piece(state) => state.color == *color
        }
    }

    pub(crate) fn move_piece(&mut self, from: &Position, to: &Position) {
        self.board[to.as_board_index()] = Field::Empty;
        self.board.swap(from.as_board_index(), to.as_board_index());
    }
}

impl Board {
    pub fn new() -> Board {
        let mut board = [Field::Empty; BOARD_SIZE];
        Board::init_white_row(&mut board, Board::WHITE_START_ROW,
                              [KNIGHT, ROOK, BISHOP, KING, QUEEN, BISHOP, ROOK, KNIGHT]);
        Board::init_white_row(&mut board, Board::WHITE_PAWN_ROW,
                              [Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn,]);
        Board::init_black_row(&mut board, Board::BLACK_PAWN_ROW,
                              [Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn,]);
        Board::init_black_row(&mut board, Board::BLACK_START_ROW,
                              [KNIGHT, ROOK, BISHOP, QUEEN, KING, BISHOP, ROOK, KNIGHT]);

        return Board{ board }
    }

    pub fn new_from(fields: Vec<(Field, Position)>) -> Board {
        let mut board = [Field::Empty; BOARD_SIZE];
        for field in fields {
            board[field.1.as_board_index()] = field.0;
        }
        return Board{board};
    }

    fn init_black_row(board: &mut[Field; BOARD_SIZE], row: usize, types: [Type; 8]) {
        let start = row * BOARD_WIDTH;
        for i in 0..8 {
            board[start+i] = Field::new_black(types[i])
        }
    }

    fn init_white_row(board: &mut[Field; BOARD_SIZE], row: usize, types: [Type; 8]) {
        let start = row * BOARD_WIDTH;
        for i in 0..8 {
            board[start+i] = Field::new_white(types[i])
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Field {
    Piece(PieceState),
    Empty
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Field::Piece(state) => state.fmt(f),
            Field::Empty => f.pad(" ")
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        return Field::Empty
    }
}

impl Field {

    #[inline]
    pub fn new_black(piece_type: Type) -> Field {
        return Field::Piece(PieceState{
            piece_type,
            color: Color::Black,
            moved: PieceMoved::No,
        })
    }

    #[inline]
    pub fn new_white(piece_type: Type) -> Field {
        return Field::Piece(PieceState{
            piece_type,
            color: Color::White,
            moved: PieceMoved::No
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {

    #[inline]
    pub(crate) fn delta(&self, x: isize, y: isize) -> Position {
        return Position {x: (self.x as isize + x) as usize, y: (self.y as isize + y) as usize }
    }

    #[inline]
    pub(crate) fn delta_if_valid(&self, x: isize, y: isize) -> Result<Position, ErrorKind> {
        let new_x = self.x as isize + x;
        let new_y = self.y as isize + y;
        return if new_x >= 0 && new_x < BOARD_WIDTH as isize &&
            new_y >= 0 && new_y < BOARD_WIDTH as isize {
            Result::Ok(Position {x: new_x as usize, y: new_y as usize})
        } else {
            Result::Err(ErrorKind::CoordinatesOutsideBoard)
        }

    }

    #[inline]
    pub(crate) fn delta_x(&self, x: isize) -> Position {
        return Position {x: (self.x as isize + x) as usize, y: self.y}
    }

    #[inline]
    pub(crate) fn delta_y(&self, y: isize) -> Position {
        return Position {x: self.x, y: (self.y as isize + y) as usize }
    }

    #[inline]
    fn as_board_index(&self) -> usize {
        return self.y * BOARD_WIDTH + self.x
    }

}
