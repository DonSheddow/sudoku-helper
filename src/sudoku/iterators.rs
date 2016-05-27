use sudoku::{Slot, Grid};

pub struct RowIterator<'a> {
    iter: ::std::slice::Iter<'a, [Slot; 9]>,
}

impl<'a> RowIterator<'a> {
    pub fn new(grid: &'a Grid) -> RowIterator {
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
    pub fn new(grid: &'a Grid) -> ColumnIterator<'a> {
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
    pub fn new(grid: &'a Grid) -> BlockIterator<'a> {
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

pub struct UnitIterator<'a> {
    rows: RowIterator<'a>,
    columns: ColumnIterator<'a>,
    blocks: BlockIterator<'a>,
}

impl<'a> UnitIterator<'a> {
    pub fn new(grid: &'a Grid) -> UnitIterator {
        UnitIterator {
            rows: RowIterator::new(grid),
            columns: ColumnIterator::new(grid),
            blocks: BlockIterator::new(grid),
        }
    }
}

impl<'a> Iterator for UnitIterator<'a> {
    type Item = Vec<&'a Slot>;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.next().or_else(||
            self.columns.next().or_else(||
                self.blocks.next()
            )
        )
    }
}
