#![no_std]
#![forbid(unsafe_code)]

#[macro_use] extern crate alloc;

use core::fmt;
use alloc::vec::Vec;

use z3::{Context, Solver, SatResult};
use z3::ast::{Ast, Bool, Int};

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
    pub fn inflate(&self, padding: usize) -> Self {
        let mut res = Self::new(self.rows + 2 * padding, self.cols + 2 * padding);
        for row in 0..self.rows {
            for col in 0..self.cols {
                res.set(row + padding, col + padding, self.get(row, col).unwrap()).unwrap();
            }
        }
        res
    }
    pub fn forward(&self, steps: usize) -> Self {
        let mut res = self.clone();
        for _ in 0..steps {
            let mut counts = vec![0; res.rows * res.cols];
            for row in 0..res.rows {
                for col in 0..res.cols {
                    if res.data[row * res.cols + col] {
                        for c_row in row.saturating_sub(1)..(row + 2).min(res.rows) {
                            for c_col in col.saturating_sub(1)..(col + 2).min(res.cols) {
                                counts[c_row * res.cols + c_col] += 1;
                            }
                        }
                    }
                }
            }
            res = Conway { rows: res.rows, cols: res.cols, data: res.data.iter().zip(counts).map(|(live, count)| (*live && count == 4) || count == 3).collect() };
        }
        res
    }
    pub fn backward(&self, steps: usize) -> Option<Self> {
        let context = Context::new(&Default::default());

        let zero = Int::from_u64(&context, 0);
        let one = Int::from_u64(&context, 1);
        let three = Int::from_u64(&context, 3);
        let four = Int::from_u64(&context, 4);

        let vars = {
            let mut vars = Vec::with_capacity(self.rows * self.cols);
            for row in 0..self.rows {
                for col in 0..self.cols {
                    vars.push(Bool::new_const(&context, format!("d{row},{col}")));
                }
            }
            vars
        };

        let mut res = vars.clone();
        for _ in 0..steps {
            let mut counts = vec![zero.clone(); self.rows * self.cols];
            for row in 0..self.rows {
                for col in 0..self.cols {
                    for c_row in row.saturating_sub(1)..(row + 2).min(self.rows) {
                        for c_col in col.saturating_sub(1)..(col + 2).min(self.cols) {
                            counts[c_row * self.cols + c_col] += res[row * self.cols + col].ite(&one, &zero);
                        }
                    }
                }
            }
            res = res.iter().zip(counts).map(|(live, count)| (live & count._eq(&four) | count._eq(&three))).collect();
        }

        let s = Solver::new(&context);
        for (res, actual) in res.iter().zip(&self.data) {
            if *actual {
                s.assert(res);
            } else {
                let res = res.not();
                s.assert(&res);
            }
        }

        match s.check() {
            SatResult::Sat => {
                let model = s.get_model().unwrap();
                Some(Self { rows: self.rows, cols: self.cols, data: vars.iter().map(|v| model.eval(v, true).unwrap().as_bool().unwrap()).collect() })
            }
            SatResult::Unsat => None,
            SatResult::Unknown => unreachable!(),
        }
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
