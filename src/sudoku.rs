extern crate serde_json;

use serde_json::Value;

use std::io::prelude::*;
use std::error;
use std::convert;
use std::fmt;
use std::num::ParseIntError;

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

#[derive(Debug)]
pub enum SudokuParseError {
    Json(serde_json::error::Error),
    IntParsing(ParseIntError),
    Syntax(&'static str),
}

impl fmt::Display for SudokuParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SudokuParseError::Json(ref error) => fmt::Display::fmt(error, fmt),
            SudokuParseError::IntParsing(ref error) => fmt::Display::fmt(error, fmt),
            SudokuParseError::Syntax(ref s) => fmt::Display::fmt(s, fmt),
        }
    }
}

impl error::Error for SudokuParseError {
    fn description(&self) -> &str {
        match *self {
            SudokuParseError::Json(ref e) => error::Error::description(e),
            SudokuParseError::IntParsing(ref e) => error::Error::description(e),
            SudokuParseError::Syntax(s) => s,
        }
    }
}

impl From<serde_json::error::Error> for SudokuParseError {
    fn from(error: serde_json::error::Error) -> SudokuParseError {
        SudokuParseError::Json(error)
    }
}

impl From<ParseIntError> for SudokuParseError {
    fn from(error: ParseIntError) -> SudokuParseError {
        SudokuParseError::IntParsing(error)
    }
}

impl From<&'static str> for SudokuParseError {
    fn from(error: &'static str) -> SudokuParseError {
        SudokuParseError::Syntax(error)
    }
}

pub struct SudokuPuzzle {
    grid: Grid,
}

impl SudokuPuzzle {

    pub fn new<T>(mut file: T) -> Result<Self, Box<error::Error>>
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

    pub fn from_json(json_str: &str) -> Result<Self, SudokuParseError> {
        let json: Value = try!(serde_json::from_str(json_str));

        let mut puzzle = SudokuPuzzle {
            grid: [[Slot::Empty; 9]; 9],
        };

        let v = try!(json.as_array().ok_or("unable to interpret JSON as array"));
        if v.len() != 9 {
            return Err(convert::From::from("array must contain exactly 9 rows"));
        }
        for (row_idx, row_value) in v.iter().enumerate() {
            let str_row = try!(row_value.as_string().ok_or("rows must be strings"));
            let mut numbers = str_row.split(',');
            for col_idx in 0..9 {
                let str_num = try!(numbers.next().ok_or("not enough numbers in row")).trim();
                if str_num == "_" {
                    puzzle.grid[row_idx][col_idx] = Slot::Empty;
                }
                else {
                    let num: u8 = try!(str_num.parse());
                    if !(1 <= num && num <= 9) {
                        return Err(convert::From::from("number must be between 1 and 9 inclusive"));
                    }
                    puzzle.grid[row_idx][col_idx] = Slot::Filled(num);
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
