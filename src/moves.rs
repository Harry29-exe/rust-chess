use crate::board::{Board, BOARD_WIDTH, BoardField, BoardPosition};
use crate::piece::{PieceColor, PieceMoved, PieceState, PieceType};
use crate::piece::PieceColor::{Black, White};

pub fn get_moves(board: &Board, pos: &BoardPosition) -> Vec<BoardPosition> {
    let mut moves: Vec<BoardPosition> = Vec::new();
    let piece = board.value_at(pos);
    return match piece {
        BoardField::Empty => vec![],
        BoardField::Piece(state) => {
            let mut move_service = PossibleMovesService::new(board, pos, &state, &mut moves);
            get_piece_moves(&mut move_service);
            moves
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

#[inline]
fn get_piece_moves(move_service: &mut PossibleMovesService) {
    match move_service.piece_state.piece_type {
        PieceType::Pawn => get_moves_pawn(move_service),
        PieceType::KNIGHT => get_moves_knight(move_service),
        PieceType::BISHOP => get_moves_bishop(move_service),
        PieceType::QUEEN => get_moves_queen(move_service),
        _ => panic!("not implemented")
    };
}

#[inline]
fn get_moves_pawn(move_service: &mut PossibleMovesService<'_>) {
    let pos_forward = move_service.piece_pos.delta_y(move_service.forward_y);
    if move_service.board.is_empty(&pos_forward) {
        move_service.moves.push(pos_forward);
        let pos_forward2 = move_service.piece_pos.delta_y(move_service.forward_y+move_service.forward_y);
        if move_service.piece_state.moved == PieceMoved::No && move_service.board.is_empty(&pos_forward2) {
            move_service.moves.push(pos_forward2);
        }
    }


    if move_service.piece_pos.x > 0 {
        let left = move_service.piece_pos.delta(-1, move_service.forward_y);
        if move_service.board.is_color(&left, &move_service.opposite_color) {
            move_service.moves.push(left)
        }
    }

    if move_service.piece_pos.x < BOARD_WIDTH - 1 {
        let right = move_service.piece_pos.delta(1, move_service.forward_y);
        if move_service.board.is_color(&right, &move_service.opposite_color) {
            move_service.moves.push(right)
        }
    }
}

#[inline]
fn get_moves_knight(move_service: &mut PossibleMovesService) {
    get_moves_horizontal(move_service);
    get_moves_vertical(move_service);
}

#[inline]
fn get_moves_bishop(move_service: &mut PossibleMovesService) {
    get_moves_horizontal(move_service);
}

#[inline]
fn get_moves_queen(move_service: &mut PossibleMovesService) {
    get_moves_knight(move_service);
    get_moves_bishop(move_service);
}

#[inline]
fn get_moves_vertical(move_service: &mut PossibleMovesService) {
    get_moves_loop(move_service, 0, 1);
    get_moves_loop(move_service, 0, -1);
}

#[inline]
fn get_moves_horizontal(move_service: &mut PossibleMovesService) {
    get_moves_loop(move_service, 1, 0);
    get_moves_loop(move_service, -1, 0);
}

#[inline]
fn get_moves_diagonal(move_service: &mut PossibleMovesService) {
    get_moves_loop(move_service, -1, -1);
    get_moves_loop(move_service, -1, 1);
    get_moves_loop(move_service, 1, -1);
    get_moves_loop(move_service, 1, 1);
}

#[inline]
fn get_moves_loop(move_service: &mut PossibleMovesService,
                  delta_x: isize,
                  delta_y: isize,
) {
    let mut previous_pos = move_service.piece_pos.clone();

    loop {
        let new_pos_result = previous_pos.delta_if_valid(delta_x, delta_y);
        if new_pos_result.is_err() {
            break;
        }

        let new_pos = new_pos_result.unwrap();
        if move_service.board.is_empty(&new_pos) {
            move_service.moves.push(new_pos.clone());
            previous_pos = new_pos;
            continue;
        }

        if move_service.board.is_color(&new_pos, &move_service.opposite_color) {
            move_service.moves.push(new_pos);
        }
        break;
    }
}
