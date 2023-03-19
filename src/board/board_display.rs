use std::fmt;
use std::fmt::Formatter;
use crate::board::{Board, BOARD_SIZE, BOARD_WIDTH};

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut is_black = true;
        for i in (0..BOARD_SIZE).rev() {
            if is_black {
                f.pad("\x1B[48;2;80;80;88m").expect("can not print to console?");
            } else {
                f.pad("\x1B[48;2;106;162;226m").expect("can not print to console?");
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