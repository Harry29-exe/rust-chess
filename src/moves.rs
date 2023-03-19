use crate::board::{Board, BOARD_SIZE, BOARD_WIDTH, Field, Position};
use crate::errors::ErrorKind;
use crate::board::piece::PieceMoved;
use crate::board::piece::PieceState;
use crate::board::piece::Type;
use crate::board::piece::Color;
use crate::board::piece::Color::{Black, White};

pub fn get_moves(board: &Board, pos: &Position, moves: &mut Vec<Position>){
    let piece = board.value_at(pos);
    match piece {
        Field::Empty => {},
        Field::Piece(state) => {
            let mut move_service = PossibleMovesService::new(board, pos, state, moves);
            move_service.get_piece_moves();
        }
    };
}

struct PossibleMovesService<'s> {
    moves: &'s mut Vec<Position>,
    board: &'s Board,
    piece_pos: &'s Position,
    piece_state: &'s PieceState,
    opposite_color: Color,
    forward_y: isize
}

impl PossibleMovesService<'_> {
    fn new<'s>(board: &'s Board, piece_pos: &'s Position, piece_state: &'s PieceState, vec: &'s mut Vec<Position>) -> PossibleMovesService<'s> {
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
            Type::Pawn => self.get_moves_pawn(),
            Type::KNIGHT => self.get_moves_knight(),
            Type::BISHOP => self.get_moves_bishop(),
            Type::QUEEN => self.get_moves_queen(),
            Type::ROOK => self.get_moves_rook(),
            Type::KING => self.get_moves_king(),
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
    fn get_moves_king(&mut self) {
        let pos = self.piece_pos;
        if pos.x > 0 {
            let left_pos = pos.delta_x(-1);
            self.add_pos_if_empty_or_enemy(left_pos);

            if pos.y > 0 {
                let left_bottom_pos = pos.delta(-1, -1);
                self.add_pos_if_empty_or_enemy(left_bottom_pos);

                let bottom_pos = pos.delta_y(-1);
                self.add_pos_if_empty_or_enemy(bottom_pos);
            }

            if pos.y < BOARD_WIDTH {
                let left_top_pos = pos.delta(-1, 1);
                self.add_pos_if_empty_or_enemy(left_top_pos);

                let top_pos = pos.delta_y(1);
                self.add_pos_if_empty_or_enemy(top_pos);
            }
        }

        if pos.x < BOARD_SIZE {
            let right = pos.delta_x(1);
            self.add_pos_if_empty_or_enemy(right);

            if pos.y > 0 {
                let right_bottom_pos = pos.delta(1, -1);
                self.add_pos_if_empty_or_enemy(right_bottom_pos);
            }
            if pos.y < BOARD_SIZE {
                let right_bottom_pos = pos.delta(1, 1);
                self.add_pos_if_empty_or_enemy(right_bottom_pos);
            }
        }
    }

    #[inline]
    fn get_moves_rook(&mut self) {
        let pos = self.piece_pos;
        self.try_add_pos_if_empty_or_enemy(pos.delta_if_valid(-2, 1));
        self.try_add_pos_if_empty_or_enemy(pos.delta_if_valid(-2, -1));
        self.try_add_pos_if_empty_or_enemy(pos.delta_if_valid(-1, -2));
        self.try_add_pos_if_empty_or_enemy(pos.delta_if_valid(1, -2));
        self.try_add_pos_if_empty_or_enemy(pos.delta_if_valid(2, -1));
        self.try_add_pos_if_empty_or_enemy(pos.delta_if_valid(2, 1));
        self.try_add_pos_if_empty_or_enemy(pos.delta_if_valid(1, 2));
        self.try_add_pos_if_empty_or_enemy(pos.delta_if_valid(-1, 2));
    }

    #[inline]
    fn get_moves_knight(&mut self) {
        self.get_moves_horizontal();
        self.get_moves_vertical();
    }

    #[inline]
    fn get_moves_bishop(&mut self) {
        self.get_moves_diagonal();
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
        let mut new_pos: Position;
        let mut new_pos_result: Result<Position, ErrorKind>;

        loop {
            new_pos_result = previous_pos.delta_if_valid(delta_x, delta_y);
            match new_pos_result {
                Err(..) => break,
                Ok(pos) => new_pos = pos
            }

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

impl PossibleMovesService<'_> {
    #[inline]
    fn add_pos_if_empty_or_enemy(&mut self, pos: Position) {
        if self.board.is_empty_or_color(&pos, &self.opposite_color) {
            self.moves.push(pos);
        }
    }

    #[inline]
    fn try_add_pos_if_empty_or_enemy(&mut self, pos_result: Result<Position, ErrorKind>) {
        match pos_result {
            Err(..) => {},
            Ok(pos) => self.add_pos_if_empty_or_enemy(pos)
        }
    }
}