#![feature(globs)]
// Tetris in Rust using ncurses.
// I'll redraw screen every frame.
// I'd almost rather use Haskell, but I want better
// ncurses bindings and GUI use. Also, to see how 
// vectors are now. But I'll have to look stuff up again.


// Shapes
// boundary detection
// game grid
// scoring behavior
// level increase (speed)
// key bindings
// rotate

extern crate ncurses;

use ncurses::*;
//use std::rand;

//Data types for each cell: the cell itself, status, and color
#[deriving( PartialEq, Show)]
enum Cell { 
    Filled,
    Empty,
}
    


//Data types for each piece. Should I track as pieces or not 
//while on the board? In buffer, for sure.
enum Piece {Straight, Bent_L, Bent_R, Zee_L, Zee_R, Tee, Square, }

//Data type for building game board
struct Game {
    score: i32,
    board: Vec<Vec<Cell>>,
    buffer: Vec<Piece>,
}

impl Game {
    
    fn new() -> Game {
        let cell = Empty;
        let mut board = Vec::new();
        for _ in range(0u,20) {
            let mut row: Vec<Cell> = Vec::new();
            for _ in range (0u,10) {
                row.push(cell);
            }
            board.push(row);
        }
        Game {score: 0, board: board, buffer: vec![Straight]}
    }

    fn draw_board(&self) -> String {
        let mut string_board: Vec<&str> = Vec::new();
        for i in range(0u,20) {
            string_board.push("||");
            for j in range(0u,10) {
                if self.board[i][j] == Filled {
                    string_board.push("*");
                }
                else {string_board.push(" ");}
            }
            string_board.push("||\n")
        }
        string_board.push("==============");
        string_board.concat()
    }
}

fn main() {
    let game = Game::new();
    println!("{}", game.board);
    println!("{}", game.draw_board());
    println!("Score: {}", game.score);


//    initscr();
//    printw("Tetris!\nby Walther Chen \n\n\n");
//    refresh();
//    napms(10000);
//    endwin();

}
