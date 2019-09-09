use std::collections::HashMap;
use std::error::Error;

use crate::cell::Cell;
use crate::dataframe::DataFrame;
use crate::series::Series;

pub trait Operations {
    fn new(vec: Vec<Vec<Cell>>, labels: Vec<&str>) -> DataFrame;
    fn push(&mut self, element: Vec<Cell>);
    fn series(&mut self, index: usize) -> &mut Series<Cell>;
    /// Get selected column by using label name
    fn by(&mut self, label: &str) -> Option<&mut Series<Cell>>;
    /// Get selected column by using label name
    fn many(&mut self, labels: Vec<&str>) -> Vec<Series<Cell>>;
    fn map(&mut self, col: &str, obj: HashMap<&str, u32>) -> DataFrame;
    /// Concatenate two data frames
    fn concat(&mut self, df: DataFrame) -> DataFrame;
    /// Drop column by label from Data Frame
    fn drop(&mut self, label: Vec<&str>) -> Option<DataFrame>;
    /// Drop column by position from Data Frame
    fn drop_idx(&mut self, position: usize) -> Option<DataFrame>;
    fn contains(self, label: &str) -> bool;
    fn read_csv(file_name: String) -> Result<DataFrame, Box<dyn Error>>;
}
