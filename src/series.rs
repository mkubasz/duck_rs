use ndarray::{Array1, Array};
use std::collections::HashSet;
use crate::element::Element;

#[derive(Debug, Clone)]
pub struct Series {
    pub label: String,
    pub data: Vec<String>

}

pub trait SeriesImpl {
    fn new() -> Series;
    fn push(&mut self, element: Element);
    fn unique(&mut self) -> Series;
}

#[derive(Debug, Clone)]
pub enum TSeries {
    Text(Series),
}

impl SeriesImpl for Series {
    fn new() -> Series {
        Series {
            label: "".to_string(),
            data: Vec::new()
        }
    }

    fn push(&mut self, element: Element) {
        match element {
            Element::Text(val) => self.data.push(val.clone()),
            Element::Integer(val) => self.data.push(format!("{}", val.clone())),
            Element::Float(val) => self.data.push(format!("{}", val.clone())),
            Element::Bool(val) => self.data.push(format!("{}", val.clone())),
        }
    }

    fn unique(&mut self) -> Series {
        let mut unique_values = HashSet::new();
        self.data.iter().for_each(|e| {
            unique_values.insert(e.clone());
        });
        let mut column = Series::new();
        unique_values.iter().for_each(
            |el| column.data.push(el.clone())
        );
        column
    }
}
