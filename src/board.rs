use crate::piece::Piece;
use crate::piece::Piece::{BISHOP, KING, KNIGHT, Blank, Pawn, QUEEN, ROOK};
use crate::piece::PieceColor::{Black, White};
use crate::piece::PieceMoved::No;

pub const BOARD_WIDTH: usize = 8;
pub const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;


pub struct Board {
    board: [Piece; BOARD_SIZE]
}

impl Board {
    pub(crate) fn new() -> Board {
        return Board{
            board: [
                ROOK(White, No), KNIGHT(White), BISHOP(White), QUEEN(White), KING(White, No), BISHOP(White), KNIGHT(White), ROOK(White, No),
                Pawn(White, No), Pawn(White, No), Pawn(White, No), Pawn(White, No), Pawn(White, No), Pawn(White, No), Pawn(White, No), Pawn(White, No),
                Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank,
                Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank,
                Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank,
                Blank, Blank, Blank, Blank, Blank, Blank, Blank, Blank,
                Pawn(Black, No), Pawn(Black, No), Pawn(Black, No), Pawn(Black, No), Pawn(Black, No), Pawn(Black, No), Pawn(Black, No), Pawn(Black, No),
                ROOK(Black, No), KNIGHT(Black), BISHOP(Black), QUEEN(Black), KING(Black, No), BISHOP(Black), KNIGHT(Black), ROOK(Black, No),
            ]
        }
    }

    #[inline]
    pub(crate) fn value(&self, x: usize, y: usize) -> Piece {
        return self.board[y*BOARD_WIDTH + x]
    }

    #[inline]
    pub(crate) fn value_at(&self, pos: &BoardPosition) -> Piece {
        return self.board[pos.y*BOARD_WIDTH + pos.x]
    }
    
    #[inline]
    pub(crate) fn not_none_at(&self, pos: &BoardPosition) -> bool {
        return self.value_at(pos) != Blank
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