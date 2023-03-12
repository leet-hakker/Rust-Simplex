use crate::m::BigM;
use std::cmp;

#[derive(Debug, Eq, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    values: Vec<Vec<BigM>>,
}

impl Matrix {
    pub fn new(rows: usize, columns: usize) -> Matrix {
        Matrix {rows, columns, values: Vec::<Vec::<BigM>>::with_capacity(rows)}
    }

    pub fn new_with_data(rows: usize, columns: usize, values: Vec<Vec<BigM>>) -> Matrix {
        Matrix { rows, columns, values}
    }

    pub fn get(&self, row: usize, column: usize) -> BigM {
        self.values[row][column]
    }

    pub fn set(&mut self, row: usize, column: usize, value: BigM) -> bool {
        if row > self.rows-1 { 
            return false;
        } else if column > self.columns-1 {
            return false;
        }
        self.values[row][column] = value;
        return true;
    }
}

impl cmp::PartialEq<Matrix> for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        return self.values == other.values;
    }

    fn ne(&self, other: &Matrix) -> bool {
        return !(self.values == other.values);
    }
}