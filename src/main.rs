extern crate ws;

use ws::{listen, Sender, Handler, Message, Handshake};

use std::io::stdin;
use std::env;
use std::fs::File;
use std::io::BufReader;

mod sudoku;
use sudoku::SudokuPuzzle;
use sudoku::Slot;

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        println!("Server got message '{}'", msg);
        let s = msg.into_text().unwrap();

        let puzzle = match SudokuPuzzle::from_json(&s) {
            Ok(r) => r,
            Err(e) => { return self.out.send(format!("{}", e)) },
        };
        let solved = puzzle.solved();
        match solved {
            Some(puzzle) => self.out.send(puzzle.serialize()),
            None => self.out.send("No solution found"),
        }
    }

    fn on_open(&mut self, shake: Handshake) -> ws::Result<()> {
        println!("Got connection from {}", shake.remote_addr().unwrap().unwrap());
        self.out.send("Hello, world!")
    }

}

fn main() {


    listen("127.0.0.1:3012", |out| {
        Server { out: out }
    }).unwrap();


    let stdin = stdin();
    let puzzle = match env::args().nth(1) {
        Some(path) => SudokuPuzzle::from_file(BufReader::new(File::open(path).unwrap())).unwrap(),
        None => SudokuPuzzle::from_file(stdin.lock()).unwrap(),
    };

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
