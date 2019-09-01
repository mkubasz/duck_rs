use ndarray::{Array1, Array};
use std::collections::HashSet;
use crate::element::Element;

#[derive(Debug, Clone)]
pub struct Series {
    pub label: String,
    pub data: Vec<Element>
}

pub trait SeriesImpl {
    fn new() -> Series;
    fn values(&self) -> Array1<f32>;
    fn push(&mut self, element: Element);
    fn unique(&mut self) -> Series;
    fn len(self) -> usize;
}

impl SeriesImpl for Series {
    fn new() -> Series {
        Series {
            label: "".to_string(),
            data: Vec::new()
        }
    }

    /// Convert Data Frame to ndarray
    fn values(&self) -> Array1<f32> {
        let mut arr = Array::default(self.data.len());
        for (index, element) in self.data.iter().enumerate() {
            arr[index] = element.value.parse().unwrap();
        }
        arr
    }

    fn push(&mut self, element: Element) {
        self.data.push(element);
    }

    // fn convert<T: Num>(&self) -> Column {
    //     let col = self.data.into_iter().map(|p|
    //         Element::<T>{ value: T::from(p.value) }
    //     ).collect::();
    //     col
    // }

    /// Get unique values in columns
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

    fn len(self) -> usize {
        self.data.len()
    }
}
