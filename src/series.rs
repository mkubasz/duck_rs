use ndarray::{Array1, Array};
use std::collections::HashSet;
use crate::element::Element;
use crate::types::{DInteger, DFloat};

#[derive(Debug, Clone)]
pub struct Series<T> {
    pub label: String,
    pub data: Vec<T>
}

pub trait SeriesImpl {
    fn new() -> Series<Element>;
    fn push(&mut self, element: Element);
    fn unique(&mut self) -> Series<Element>;
    fn contains(self, label: &str) -> bool;
}

#[derive(Debug, Clone)]
pub enum TSeries {
    Text(Series<String>),
    Number(Series<DInteger>),
    Float(Series<DFloat>),
    Bool(Series<bool>)
}

impl SeriesImpl for Series<Element> {
    fn new() -> Series<Element> {
        Series {
            label: "".to_string(),
            data: Vec::new()
        }
    }

    fn push(&mut self, element: Element) {
        self.data.push(element);
    }

    fn unique(&mut self) -> Series<Element> {
        let mut unique_values: HashSet<String> = HashSet::new();
        self.data.iter().for_each(|e| {
            match e {
                Element::Text(cell) => {
                        unique_values.insert(cell.clone());
                    },
                _ => {}
            }

        });
        let mut column = Series::new();
        unique_values.iter().for_each(
            |el| column.data.push(Element::from(el.clone()))
        );
        column
    }

    fn contains(self, label: &str) -> bool {
        for el in self.data {
            match el {
                Element::Text(cell) => {
                    if cell.contains(label) {
                        return true
                    }
                }
                _ => {}
            }
        }
        return false
    }
}
