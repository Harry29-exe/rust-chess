use std::env::VarError;
use std::{array, fmt};
use std::fmt::{Formatter, Pointer};
use crate::errors::ErrorKind;
use crate::piece::{PieceType, PieceState, PieceMoved, PieceColor};
use crate::piece::PieceType::{BISHOP, KING, KNIGHT, Pawn, QUEEN, ROOK};

pub const BOARD_WIDTH: usize = 8;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;


pub struct Board {
    board: [BoardField; BOARD_SIZE]
}

impl Board {
    pub const WHITE_START_ROW: usize = 0;
    const WHITE_PAWN_ROW: usize = 1;
    pub const WHITE_FORWARD: isize = 1;
    pub const BLACK_START_ROW: usize = 7;
    const BLACK_PAWN_ROW: usize = 6;
    pub const BLACK_FORWARD: isize = -1;

    #[inline]
    pub(crate) fn value(&self, x: usize, y: usize) -> &BoardField {
        return &self.board[y*BOARD_WIDTH + x]
    }

    #[inline]
    pub(crate) fn value_at(&self, pos: &BoardPosition) -> &BoardField {
        return &self.board[pos.y*BOARD_WIDTH + pos.x]
    }
    
    #[inline]
    pub(crate) fn is_color(&self, pos: &BoardPosition, color: &PieceColor) -> bool {
        return match self.value_at(pos) {
            BoardField::Empty => false,
            BoardField::Piece(state) => state.color == *color
        }
    }

    #[inline]
    pub(crate) fn is_empty(&self, pos: &BoardPosition) -> bool {
        return self.value_at(pos) == &BoardField::Empty
    }

    pub(crate) fn move_piece(&mut self, from: &BoardPosition, to: &BoardPosition) {
        self.board[to.as_1d_array_index()] = BoardField::Empty;
        self.board.swap(from.as_1d_array_index(), to.as_1d_array_index());
    }
}

impl Board {
    pub(crate) fn new() -> Board {
        let mut board = [BoardField::Empty; BOARD_SIZE];
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

    fn init_black_row(board: &mut[BoardField; BOARD_SIZE], row: usize, types: [PieceType; 8]) {
        let start = row * BOARD_WIDTH;
        for i in 0..8 {
            board[start+i] = BoardField::new_black(types[i])
        }
    }

    fn init_white_row(board: &mut[BoardField; BOARD_SIZE], row: usize, types: [PieceType; 8]) {
        let start = row * BOARD_WIDTH;
        for i in 0..8 {
            board[start+i] = BoardField::new_white(types[i])
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut is_black = true;
        for i in (0..BOARD_SIZE).rev() {
            if is_black {
                f.pad("\x1B[48;2;80;80;80m").expect("can not print to console?");
            } else {
                f.pad("\x1B[48;2;200;200;200m").expect("can not print to console?");
            }
            self.board[i].fmt(f).expect("can not print to console?");
            if i % BOARD_WIDTH == 0 {
                f.pad("\x1B[0m\n").expect("can not print to console?");
                is_black = !is_black;
            }
            is_black = !is_black;
        }
        f.pad("\x1B[0m")
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum BoardField {
    Piece(PieceState),
    Empty
}

impl fmt::Display for BoardField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BoardField::Piece(state) => state.fmt(f),
            BoardField::Empty => f.pad(" ")
        }
    }
}

impl Default for BoardField {
    fn default() -> Self {
        return BoardField::Empty
    }
}

impl BoardField {

    #[inline]
    pub fn new_black(piece_type: PieceType) -> BoardField {
        return BoardField::Piece(PieceState{
            piece_type,
            color: PieceColor::Black,
            moved: PieceMoved::No,
        })
    }

    #[inline]
    pub fn new_white(piece_type: PieceType) -> BoardField {
        return BoardField::Piece(PieceState{
            piece_type,
            color: PieceColor::White,
            moved: PieceMoved::No
        })
    }

    pub fn new_empty() -> BoardField {
        return BoardField::Empty
    }

}

#[derive(Clone)]
pub struct BoardPosition {
    pub x: usize,
    pub y: usize,
}

impl BoardPosition {

    #[inline]
    pub(crate) fn delta(&self, x: isize, y: isize) -> BoardPosition {
        return BoardPosition{x: (self.x as isize + x) as usize, y: (self.y as isize + y) as usize }
    }

    #[inline]
    pub(crate) fn delta_if_valid(&self, x: isize, y: isize) -> Result<BoardPosition, ErrorKind> {
        let new_x = self.x as isize + x;
        let new_y = self.y as isize + y;
        return if new_x >= 0 && new_x < BOARD_WIDTH as isize &&
            new_y >= 0 && new_y < BOARD_WIDTH as isize {
            Result::Ok(BoardPosition{x: new_x as usize, y: new_y as usize})
        } else {
            Result::Err(ErrorKind::CoordinatesOutsideBoard)
        }

    }

    #[inline]
    pub(crate) fn delta_x(&self, x: isize) -> BoardPosition {
        return BoardPosition{x: (self.x as isize + x) as usize, y: self.y}
    }

    #[inline]
    pub(crate) fn delta_y(&self, y: isize) -> BoardPosition {
        return BoardPosition{x: self.x, y: (self.y as isize + y) as usize }
    }

    #[inline]
    fn as_1d_array_index(&self) -> usize {
        return self.y * BOARD_WIDTH + self.x
    }

}
