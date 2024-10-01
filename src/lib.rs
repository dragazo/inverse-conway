#![no_std]
#![forbid(unsafe_code)]

#[macro_use] extern crate alloc;

use core::fmt;
use alloc::vec::Vec;

#[derive(Clone, PartialEq, Eq)]
pub struct Conway {
    rows: usize,
    cols: usize,
    data: Vec<bool>,
}
impl Conway {
    pub fn new(mut rows: usize, mut cols: usize) -> Self {
        if rows == 0 || cols == 0 {
            (rows, cols) = (0, 0);
        }

        Self { rows, cols, data: vec![false; rows * cols] }
    }
    pub fn get(&self, row: usize, col: usize) -> Result<bool, ()> {
        if row < self.rows && col < self.cols {
            Ok(self.data[row * self.cols + col])
        } else {
            Err(())
        }
    }
    pub fn set(&mut self, row: usize, col: usize, value: bool) -> Result<(), ()> {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = value;
            Ok(())
        } else {
            Err(())
        }
    }
    pub fn forward(&self) -> Self {
        let mut counts = vec![0; self.rows * self.cols];
        for row in 0..self.rows {
            for col in 0..self.cols {
                if self.data[row * self.cols + col] {
                    for c_row in row.saturating_sub(1)..(row + 2).min(self.rows) {
                        for c_col in col.saturating_sub(1)..(col + 2).min(self.cols) {
                            counts[c_row * self.cols + c_col] += 1;
                        }
                    }
                }
            }
        }
        Self { rows: self.rows, cols: self.cols, data: self.data.iter().zip(counts).map(|(live, count)| (*live && count == 4) || count == 3).collect() }
    }
}
impl fmt::Debug for Conway {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.rows == 0 || self.cols == 0 {
            return f.write_str("[]");
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                f.write_str(if self.data[row * self.cols + col] { "■" } else { "□" })?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}
