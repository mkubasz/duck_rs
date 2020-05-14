use std::iter::Product;

use num::{PrimInt, Signed};

use crate::dataframe::DataFrame;
use crate::algebraic::matrix::Matrix;
use crate::cell::Cell;

pub trait Science {
    /// One hot encoding - Convert string values to binary value
    fn get_dummies(&mut self, label: &str) -> DataFrame;

    fn from_vec<T>(vec: Vec<Vec<T>>, labels: Vec<&str>) -> DataFrame
        where T: PrimInt + Signed + Product;

    /// From rows to data frame
    fn from_rows(rows: Vec<Vec<Cell>>, labels: Vec<String>) -> Option<DataFrame>;

    fn to_matrix(&self) -> Option<Matrix>;
}