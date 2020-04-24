use std::collections::HashSet;

use crate::cell::Cell;
use crate::types::{DFloat, DInteger, DFloat64};

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Series<T> {
    pub label: String,
    pub data: Vec<T>
}

pub trait SeriesImpl {
    fn new() -> Series<Cell>;
    fn push(&mut self, element: Cell);
    fn unique(&mut self) -> Series<Cell>;
    fn contains(self, label: &str) -> bool;
}

#[derive(Debug, Clone)]
pub enum TSeries {
    Text(Series<String>),
    Number(Series<DInteger>),
    Float(Series<DFloat>),
    Float64(Series<DFloat64>),
    Bool(Series<bool>)
}

impl SeriesImpl for Series<Cell> {
    fn new() -> Series<Cell> {
        Series {
            label: "".to_string(),
            data: Vec::new()
        }
    }

    fn push(&mut self, element: Cell) {
        self.data.push(element);
    }

    fn unique(&mut self) -> Series<Cell> {
        let mut unique_values: HashSet<String> = HashSet::new();
        self.data.iter().for_each(|element| {
            match element {
                Cell::Text(cell) => {
                        unique_values.insert(cell.clone());
                    },
                _ => {}
            }

        });
        let mut column = Series::new();
        unique_values.iter().for_each(
            |el| column.data.push(Cell::from(el.clone()))
        );
        column
    }

    fn contains(self, label: &str) -> bool {
        for element in self.data {
            match element {
                Cell::Text(cell) => {
                    if cell.contains(label) {
                        return true;
                    }
                }
                _ => {}
            }
        }
        return false;
    }
}
