use std::io::prelude::*;
use std::io::stdin;
use std::error::Error;
use std::convert;
use std::env;
use std::fs::File;
use std::io::BufReader;

type Grid = [[Slot; 9]; 9];

type RowIterator<'a> = ::std::slice::Iter<'a, [Slot; 9]>;

struct ColumnIterator<'a> {
    grid: &'a Grid,
    column: u8,
}

impl<'a> ColumnIterator<'a> {
    fn new(grid: &'a Grid) -> ColumnIterator<'a> {
        ColumnIterator { grid: grid, column: 0 }
    }
}

impl<'a> Iterator for ColumnIterator<'a> {
    type Item = Vec<&'a Slot>;

    fn next(&mut self) -> Option<Self::Item> {

        if self.column >= 9 {
            return None;
        }

        let mut v = vec![];
        for row in 0..9 {
            v.push(&self.grid[row as usize][self.column as usize]);
        }

        self.column += 1;

        Some(v)
    }
}


struct BlockIterator<'a> {
    grid: &'a Grid,
    row: u8,
    column: u8,
}

impl<'a> BlockIterator<'a> {
    fn new(grid: &'a Grid) -> BlockIterator<'a> {
        BlockIterator { grid: grid, row: 0, column: 0 }
    }
}

impl<'a> Iterator for BlockIterator<'a> {
    type Item = Vec<&'a Slot>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.column >= 9 {
            self.column = 0;
            self.row += 3;
        }
        if self.row >= 9 {
            return None;
        }

        let mut v = vec![];
        for i in 0..3 {
            for j in 0..3 {
                v.push(&self.grid[self.row as usize + i][self.column as usize + j]);
            }
        }

        self.column += 3;

        Some(v)
    }
}


#[derive(Copy, Clone)]
enum Slot {
    Empty,
    Filled(u8),
}

struct SudokuPuzzle {
    grid: Grid,
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

    fn rows(&self) -> RowIterator {
        self.grid.iter()
    }

    fn columns(&self) -> ColumnIterator {
        ColumnIterator::new(&self.grid)
    }

    fn blocks(&self) -> BlockIterator {
        BlockIterator::new(&self.grid)
    }

    fn print(&self) {
        let row_separator = ["-"; 9];
        let top_bot_sep = row_separator.join("-");
        println!("{}", top_bot_sep);
        let mut formatted_rows = vec![];
        for row in self.rows() {
            let mut str_row = "|".to_owned();
            for &slot in row.iter() {
                let s = match slot {
                    Slot::Empty => " |".to_owned(),
                    Slot::Filled(n) => format!("{}|", n),
                };
                str_row.push_str(&s);
            }
            str_row.push('\n');
            formatted_rows.push(str_row);
        }
        let between_sep = format!("|{}\n", row_separator.join("+"));
        print!("{}", formatted_rows.join(&between_sep));
        println!("{}", top_bot_sep);
    }
    
}


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
