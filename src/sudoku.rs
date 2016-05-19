use std::io::prelude::*;
use std::error::Error;
use std::convert;

type Grid = [[Slot; 9]; 9];

pub struct RowIterator<'a> {
    iter: ::std::slice::Iter<'a, [Slot; 9]>,
}

impl<'a> RowIterator<'a> {
    fn new(grid: &'a Grid) -> RowIterator {
        RowIterator { iter: grid.iter() }
    }
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Vec<&'a Slot>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|a| a.iter().collect::<Vec<_>>())
    }
}

pub struct ColumnIterator<'a> {
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


pub struct BlockIterator<'a> {
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
pub enum Slot {
    Empty,
    Filled(u8),
}

pub struct SudokuPuzzle {
    grid: Grid,
}

impl SudokuPuzzle {

    pub fn new<T>(mut file: T) -> Result<Self, Box<Error>>
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

    pub fn rows(&self) -> RowIterator {
        RowIterator::new(&self.grid)
    }

    pub fn columns(&self) -> ColumnIterator {
        ColumnIterator::new(&self.grid)
    }

    pub fn blocks(&self) -> BlockIterator {
        BlockIterator::new(&self.grid)
    }

    pub fn print(&self) {
        let row_separator = ["-"; 9];
        let top_bot_sep = row_separator.join("-");
        println!("{}", top_bot_sep);
        let mut formatted_rows = vec![];
        for row in self.rows() {
            let mut str_row = "|".to_owned();
            for &slot in row {
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
