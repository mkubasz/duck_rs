use std::iter::Product;

use num::{PrimInt, Signed};

use crate::dataframe::DataFrame;

pub trait Science {
    /// One hot encoding - Convert string values to binary value
    fn get_dummies(&mut self, label: &str) -> DataFrame;
    fn from_vec<T>(vec: Vec<Vec<T>>, labels: Vec<&str>) -> DataFrame where T: PrimInt + Signed + Product;
}