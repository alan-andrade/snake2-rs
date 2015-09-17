use std::fmt::{Formatter, Display};

#[derive(Debug)]
enum Object {
    B, // Snake's body
    F, // Free
    H, // Snake's head
    W  // Wall
}

use Object::{W, F, H, B};

impl Display for Object {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
         fmt.write_str("X")
    }
}

fn main() {

    let board = vec!(vec!(W, W, W, W),
                     vec!(W, F, F, W),
                     vec!(W, B, H, W),
                     vec!(W, W, W, W));

    print_board(&board);
}

fn print_board(board: &[Vec<Object>]) {
    for line in board {
        for object in line {
            match object {
                &Object::W => { print!("#"); }
                &Object::F => { print!(" ") }
                &Object::H => { print!("O") }
                &Object::B => { print!("o") }
            }
        }

        println!("");
    }
}
