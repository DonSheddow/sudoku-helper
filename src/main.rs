use std::io::prelude::*;
use std::io::stdin;
use std::error::Error;
use std::convert;
use std::env;
use std::fs::File;
use std::io::BufReader;

#[derive(Copy, Clone)]
enum Slot {
    Empty,
    Filled(u8),
}

struct SudokuPuzzle {
    grid: [[Slot; 9]; 9],
}

impl SudokuPuzzle {

    fn new<T>(mut file: T) -> Result<Self, Box<Error>>
        where T: BufRead {

        let mut puzzle = SudokuPuzzle {
            grid: [[Slot::Empty; 9]; 9],
        };
        
        for row in puzzle.grid.iter_mut() {
            let mut buffer = String::new();
            try!(file.read_line(&mut buffer));
            let mut numbers = buffer.split(',');
            for slot in row.iter_mut() {
                let str_num = try!(numbers.next().ok_or("not enough numbers")).trim();
                if str_num == "_" {
                    *slot = Slot::Empty;
                }
                else {
                    let num: u8 = try!(str_num.parse());
                    if !(1 <= num && num <= 9) {
                        return Err(convert::From::from("number must be between 1 and 9 inclusive"));
                    }
                    *slot = Slot::Filled(num);
                }
            }
        }
        
        Ok(puzzle)
    }

    fn print() {
    }

}


fn main() {
    let stdin = stdin();
    let puzzle = match env::args().nth(1) {
        Some(path) => SudokuPuzzle::new(BufReader::new(File::open(path).unwrap())),
        None => SudokuPuzzle::new(stdin.lock()),
    };

    match puzzle {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: {}", e),
    }
    println!("Hello, world!");
}
