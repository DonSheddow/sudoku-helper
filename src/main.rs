extern crate ws;
extern crate serde_json;

use serde_json::builder::ObjectBuilder;
use ws::{listen, Sender, Handler, Message, Handshake};

mod sudoku;
use sudoku::SudokuPuzzle;

fn json_message<T: ToString>(msg: T) -> String {
    let value = ObjectBuilder::new()
        .insert("message", msg.to_string())
        .unwrap();
    value.to_string()
}

fn json_puzzle(puzzle: SudokuPuzzle) -> String {
    let value = ObjectBuilder::new()
        .insert("solution", puzzle.serialize())
        .unwrap();
    value.to_string()
}

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        println!("Server got message '{}'", msg);
        let s = msg.into_text().unwrap();

        let puzzle = match SudokuPuzzle::from_json(&s) {
            Ok(r) => r,
            Err(e) => { return self.out.send(json_message(e)) },
        };
        let solved = puzzle.solved();
        match solved {
            Some(puzzle) => self.out.send(json_puzzle(puzzle)),
            None => self.out.send(json_message("No solution found")),
        }
    }

    fn on_open(&mut self, shake: Handshake) -> ws::Result<()> {
        println!("Got connection from {}", shake.remote_addr().unwrap().unwrap());
        Ok(())
    }

}

fn main() {
    listen("127.0.0.1:3012", |out| {
        Server { out: out }
    }).unwrap();
}
