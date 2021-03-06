extern crate serde_json;

use self::serde_json::Value;
use self::serde_json::builder::ArrayBuilder;

use std::io::prelude::*;
use std::error;
use std::convert;
use std::fmt;
use std::num::ParseIntError;
use std::io;

mod iterators;

use self::iterators::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Slot {
    Empty,
    Filled(u8),
}

impl Slot {
    fn from_json(val: &Value) -> Result<Self, &'static str> {
        if val.is_null() {
            Ok(Slot::Empty)
        }
        else if val.is_number() {
            let num = val.as_u64().unwrap();
            if 1 <= num && num <= 9 {
                Ok(Slot::Filled(num as u8))
            }
            else {
                Err("number must be between 1 and 9 inclusive")
            }
        }
        else {
            Err("cell value must be empty or a number")
        }
    }

    fn to_json(&self) -> Value {
        match *self {
            Slot::Empty => Value::Null,
            Slot::Filled(n) => serde_json::to_value(&n),
        }
    }
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

#[derive(Clone, Debug)]
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
            let row = try!(row_value.as_array().ok_or("unable to interpret row as array"));
            if row.len() != 9 {
                return Err(convert::From::from("row must contain exactly 9 slots"));
            }
            for (col_idx, s) in row.iter().enumerate() {
                puzzle.grid[row_idx][col_idx] = try!(Slot::from_json(s));
            }
        }

        Ok(puzzle)
    }

    pub fn to_json(&self) -> Value {
        let mut value = ArrayBuilder::new();
        for row in self.grid.iter() {
            value = value.push_array(|builder| {
                let mut json_row = builder;
                for slot in row {
                    json_row = json_row.push(slot.to_json());
                }
                json_row
            });
        }

        value.unwrap()
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

    pub fn units(&self) -> UnitIterator {
        UnitIterator::new(&self.grid)
    }

    fn is_valid_unit(unit: &Unit) -> bool {
        for num in 1..10 {
            if unit.iter().filter(|&&x| *x == Slot::Filled(num)).count() > 1 {
                return false;
            }
        }
        return true;
    }

    pub fn is_valid(&self) -> bool {
        for unit in self.units() {
            if !Self::is_valid_unit(&unit) {
                return false;
            }
        }
        return true;
    }

    fn insert_lowest_fitting_num(&mut self, row: usize, col: usize, backtracking: bool) {
        let start = if backtracking {
            match self.grid[row][col] {
                Slot::Filled(n) => n + 1,
                Slot::Empty => panic!("Programming error"),
            }
        } else { 1 };
        for candidate in start..10 {
            self.grid[row][col] = Slot::Filled(candidate);
            if self.is_valid() {
                return;
            }
        }
        self.grid[row][col] = Slot::Empty;
    }

    pub fn solved(&self) -> Option<Self> {
        let mut result = self.clone();
        let mut backtrack = false;
        let mut index: isize = 0;
        while 0 <= index && index < 81 {
            let row = (index / 9) as usize;
            let col = (index % 9) as usize;
            if self.grid[row][col] == Slot::Empty {
                result.insert_lowest_fitting_num(row, col, backtrack);
                if result.grid[row][col] == Slot::Empty && !backtrack {
                    backtrack = true;
                }
                else if result.grid[row][col] != Slot::Empty && backtrack {
                    backtrack = false;
                }
            }
            if backtrack {
                index -= 1;
            }
            else {
                index += 1;
            }
        }
        if !backtrack {
            Some(result)
        }
        else {
            None
        }
    }
}
