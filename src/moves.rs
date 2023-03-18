use crate::board::{Board, BOARD_WIDTH, BoardField, BoardPosition};
use crate::piece::{PieceColor, PieceMoved, PieceState, PieceType};
use crate::piece::PieceColor::{Black, White};

pub fn get_moves(board: &Board, pos: &BoardPosition, moves: &mut Vec<BoardPosition>){
    let piece = board.value_at(pos);
    match piece {
        BoardField::Empty => {},
        BoardField::Piece(state) => {
            let mut move_service = PossibleMovesService::new(board, pos, state, moves);
            move_service.get_piece_moves();
        }
    };
}

struct PossibleMovesService<'s> {
    moves: &'s mut Vec<BoardPosition>,
    board: &'s Board,
    piece_pos: &'s BoardPosition,
    piece_state: &'s PieceState,
    opposite_color: PieceColor,
    forward_y: isize
}

impl PossibleMovesService<'_> {
    fn new<'s>(board: &'s Board, piece_pos: &'s BoardPosition, piece_state: &'s PieceState, vec: &'s mut Vec<BoardPosition>) -> PossibleMovesService<'s> {
        return PossibleMovesService{
            moves: vec,
            board,
            piece_pos,
            piece_state,
            opposite_color: piece_state.color.opposite(),
            forward_y: match piece_state.color {
                White => Board::WHITE_FORWARD,
                Black => Board::BLACK_FORWARD,
            }
        }
    }
}

impl PossibleMovesService<'_> {
    #[inline]
    fn get_piece_moves(&mut self) {
        match self.piece_state.piece_type {
            PieceType::Pawn => self.get_moves_pawn(),
            PieceType::KNIGHT => self.get_moves_knight(),
            PieceType::BISHOP => self.get_moves_bishop(),
            PieceType::QUEEN => self.get_moves_queen(),
            _ => panic!("not implemented")
        };
    }

    #[inline]
    fn get_moves_pawn(&mut self) {
        let pos_forward = self.piece_pos.delta_y(self.forward_y);
        if self.board.is_empty(&pos_forward) {
            self.moves.push(pos_forward);
            let pos_forward2 = self.piece_pos.delta_y(self.forward_y+ self.forward_y);
            if self.piece_state.moved == PieceMoved::No && self.board.is_empty(&pos_forward2) {
                self.moves.push(pos_forward2);
            }
        }

        if self.piece_pos.x > 0 {
            let left = self.piece_pos.delta(-1, self.forward_y);
            if self.board.is_color(&left, &self.opposite_color) {
                self.moves.push(left)
            }
        }

        if self.piece_pos.x < BOARD_WIDTH - 1 {
            let right = self.piece_pos.delta(1, self.forward_y);
            if self.board.is_color(&right, &self.opposite_color) {
                self.moves.push(right)
            }
        }
    }

    #[inline]
    fn get_moves_knight(&mut self) {
        self.get_moves_horizontal();
        self.get_moves_vertical();
    }

    #[inline]
    fn get_moves_bishop(&mut self) {
        self.get_moves_horizontal();
    }

    #[inline]
    fn get_moves_queen(&mut self) {
        self.get_moves_knight();
        self.get_moves_bishop();
    }

    #[inline]
    fn get_moves_vertical(&mut self) {
        self.get_moves_loop(0, 1);
        self.get_moves_loop(0, -1);
    }

    #[inline]
    fn get_moves_horizontal(&mut self) {
        self.get_moves_loop(1, 0);
        self.get_moves_loop(-1, 0);
    }

    #[inline]
    fn get_moves_diagonal(&mut self) {
        self.get_moves_loop(-1, -1);
        self.get_moves_loop(-1, 1);
        self.get_moves_loop(1, -1);
        self.get_moves_loop(1, 1);
    }

    #[inline]
    fn get_moves_loop(&mut self,
                      delta_x: isize,
                      delta_y: isize,
    ) {
        let mut previous_pos = self.piece_pos.clone();

        loop {
            let new_pos_result = previous_pos.delta_if_valid(delta_x, delta_y);
            if new_pos_result.is_err() {
                break;
            }

            let new_pos = new_pos_result.unwrap();
            if self.board.is_empty(&new_pos) {
                self.moves.push(new_pos.clone());
                previous_pos = new_pos;
                continue;
            }

            if self.board.is_color(&new_pos, &self.opposite_color) {
                self.moves.push(new_pos);
            }
            break;
        }
    }

}
