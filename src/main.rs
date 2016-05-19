use std::io::stdin;
use std::env;
use std::fs::File;
use std::io::BufReader;

mod sudoku;
use sudoku::SudokuPuzzle;
use sudoku::Slot;

fn main() {

    let stdin = stdin();
    let puzzle = match env::args().nth(1) {
        Some(path) => SudokuPuzzle::new(BufReader::new(File::open(path).unwrap())).unwrap(),
        None => SudokuPuzzle::new(stdin.lock()).unwrap(),
    };

    puzzle.print();
    
    let mut columns = puzzle.columns();
    let first_column = columns.next().unwrap();
    for &slot in first_column {
        match slot {
            Slot::Filled(n) => println!("{}", n),
            Slot::Empty => println!("_"),
        }
    }

    println!("");

    let mut rows = puzzle.rows();
    let first_row = rows.next().unwrap();
    
    for &slot in first_row {
        match slot {
            Slot::Filled(n) => print!("{}, ", n),
            Slot::Empty => print!("_, "),
        }
    }

    println!("");
    println!("");
    println!("");

    let blocks = puzzle.blocks();

    for block in blocks {
        for &slot in block {
            match slot {
                Slot::Filled(n) => print!("{}, ", n),
                Slot::Empty => print!("_, "),
            }
        }
        println!("")
    }
}
