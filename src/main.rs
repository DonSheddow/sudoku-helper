extern crate ws;

use ws::{listen, Sender, Handler, Message, Handshake};

mod sudoku;
use sudoku::SudokuPuzzle;

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
}
