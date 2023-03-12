use std::fmt;
use std::fmt::{Formatter, Pointer};
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
    pub(crate) fn value(&self, x: usize, y: usize) -> BoardField {
        return self.board[y*BOARD_WIDTH + x]
    }

    #[inline]
    pub(crate) fn value_at(&self, pos: &BoardPosition) -> BoardField {
        return self.board[pos.y*BOARD_WIDTH + pos.x]
    }
    
    #[inline]
    pub(crate) fn empty_or_color(&self, pos: &BoardPosition, color: &PieceColor) -> bool {
        return match self.value_at(pos) {
            BoardField::Empty => true,
            BoardField::Piece(state) => state.color == *color
        }
    }

    #[inline]
    pub(crate) fn empty(&self, pos: &BoardPosition) -> bool {
        return self.value_at(pos) == BoardField::Empty
    }

    pub(crate) fn move_piece(&mut self, from: &BoardPosition, to: &BoardPosition) {
        self.set_value(to, self.value_at(from));
        self.set_value(from, BoardField::Empty);
    }

    fn set_value(&mut self, pos: &BoardPosition, val: BoardField) {
        self.board[pos.y*BOARD_WIDTH + pos.x] = val
    }
    
}

impl Board {
    pub(crate) fn new() -> Board {
        let mut board = [BoardField::Empty; BOARD_SIZE];
        Board::init_white_row(&mut board, Board::WHITE_START_ROW,
                              [ROOK, KNIGHT, BISHOP, KING, QUEEN, BISHOP, KNIGHT, ROOK]);
        Board::init_white_row(&mut board, Board::WHITE_PAWN_ROW,
                              [Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn,]);
        Board::init_white_row(&mut board, Board::BLACK_PAWN_ROW,
                              [Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn,]);
        Board::init_black_row(&mut board, Board::BLACK_START_ROW,
                              [ROOK, KNIGHT, BISHOP, QUEEN, KING, BISHOP, KNIGHT, ROOK]);

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

#[derive(Copy, Clone, PartialEq)]
pub enum BoardField {
    Piece(PieceState),
    Empty
}

impl fmt::Display for BoardField {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            BoardField::Piece(state) => state.piece_type.fmt(f),
            BoardField::Empty => f.pad(" ")
        }
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
    pub(crate) fn delta_x(&self, x: isize) -> BoardPosition {
        return BoardPosition{x: (self.x as isize + x) as usize, y: self.y}
    }

    #[inline]
    pub(crate) fn delta_y(&self, y: isize) -> BoardPosition {
        return BoardPosition{x: self.x, y: (self.y as isize + y) as usize }
    }

}
