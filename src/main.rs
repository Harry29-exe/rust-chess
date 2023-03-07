mod piece;

const BOARD_WIDTH: usize = 8;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_WIDTH;



struct Board {
    board: [char; BOARD_SIZE]
}

impl Board {
    fn new() -> Board {
        return Board{
            board: [
                'r', 'k', 'b', 'q', 'K', 'b', 'k', 'r',
                'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
                'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p',
                'r', 'k', 'b', 'K', 'q', 'b', 'k', 'r',
            ]
        }
    }

    fn value(&self, x: usize, y: usize) -> char {
        return self.board[y*BOARD_WIDTH + x]
    }
}

fn main() {
    let board = Board::new();

    for row in 0..BOARD_WIDTH {
        for col in 0..BOARD_WIDTH {
            print!("{}",  board.value(col, row))
        }
        println!()
    }
    println!("Hello, world!");
}
