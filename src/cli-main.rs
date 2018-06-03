mod lib;
use lib::*;
use std::fmt::Write;
use std::io;

pub fn main() {
    println!("Hello world!");
    let mut board = Board {
        cells: [[false; CELLS_HIGH]; CELLS_WIDE],
        falling_block: None,
    };

    loop {
        println!("{}", print_board(&board));
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input).unwrap();
        match input.chars().next() {
            Some('r') => board.input(BlockCommand::R),
            Some('l') => board.input(BlockCommand::L),
            Some('e') => break,
            _ => (),
        }
        if !board.step() {
            println!("game over");
            break;
        }
    }
}

fn print_board(board: &Board) -> String {
    let cells = if let Some(falling_block) = board.falling_block {
        falling_block.insert_into_cells(board.cells)
    } else {
        board.cells
    };
    let mut board_string = String::new();
    for _ in 0..CELLS_WIDE+2 { write!(board_string, "-").unwrap(); }
    write!(board_string, "\n").unwrap();
    for y_inv in 0..CELLS_HIGH {
        let y = CELLS_HIGH - y_inv - 1;
        write!(board_string, "-").unwrap();
        for x in 0..CELLS_WIDE {
            if cells[x][y] {
                write!(board_string, "X").unwrap();
            } else {
                write!(board_string, " ").unwrap();
            }
        }
        write!(board_string, "-\n").unwrap();
    }
    for _ in 0..CELLS_WIDE+2 { write!(board_string, "-").unwrap(); }
    board_string
}