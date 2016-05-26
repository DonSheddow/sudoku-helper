extern crate serde_json;

use self::serde_json::Value;

use std::io::prelude::*;
use std::error;
use std::convert;
use std::fmt;
use std::num::ParseIntError;
use std::io;

mod iterators;

use self::iterators::*;

#[derive(Copy, Clone)]
pub enum Slot {
    Empty,
    Filled(u8),
}

pub type Grid = [[Slot; 9]; 9];

#[derive(Debug)]
pub enum SudokuParseError {
    Json(serde_json::error::Error),
    IntParsing(ParseIntError),
    Io(io::Error),
    Syntax(&'static str),
}

impl fmt::Display for SudokuParseError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SudokuParseError::Json(ref error) => fmt::Display::fmt(error, fmt),
            SudokuParseError::IntParsing(ref error) => fmt::Display::fmt(error, fmt),
            SudokuParseError::Io(ref error) => fmt::Display::fmt(error, fmt),
            SudokuParseError::Syntax(ref s) => fmt::Display::fmt(s, fmt),
        }
    }
}

impl error::Error for SudokuParseError {
    fn description(&self) -> &str {
        match *self {
            SudokuParseError::Json(ref e) => error::Error::description(e),
            SudokuParseError::IntParsing(ref e) => error::Error::description(e),
            SudokuParseError::Io(ref e) => error::Error::description(e),
            SudokuParseError::Syntax(s) => s,
        }
    }
}

macro_rules! impl_from_for_error {
    ( $error_type:ty, $variant:ident ) => {
        impl From<$error_type> for SudokuParseError {
            fn from(error: $error_type) -> SudokuParseError {
                SudokuParseError::$variant(error)
            }
        }
    };
}

impl_from_for_error!(serde_json::error::Error, Json);
impl_from_for_error!(ParseIntError, IntParsing);
impl_from_for_error!(io::Error, Io);
impl_from_for_error!(&'static str, Syntax);

pub struct SudokuPuzzle {
    grid: Grid,
}

impl SudokuPuzzle {

    pub fn from_file<T>(mut file: T) -> Result<Self, SudokuParseError>
        where T: BufRead {

        let mut puzzle = SudokuPuzzle {
            grid: [[Slot::Empty; 9]; 9],
        };

        for row in puzzle.grid.iter_mut() {
            let mut buffer = String::new();
            try!(file.read_line(&mut buffer));
            let mut numbers = buffer.split(',');
            for slot in row.iter_mut() {
                let str_num = try!(numbers.next().ok_or("not enough numbers in row")).trim();
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

}
