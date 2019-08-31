use std::fs::File;
use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::ops::{Index, IndexMut};
use ndarray::prelude::*;
use std::convert::From;
use num_traits::Num;
use std::any::Any;
use std::borrow::{Borrow, BorrowMut};

/// Basic elementary cell in data frame
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Element {
    value: String
}

impl Element {
    fn replace(&mut self, new: String) {
        self.value = new;
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    label: String,
    pub data: Vec<Element>
}

impl Column {
    fn new() -> Column {
        Column {
            label: "".to_string(),
            data: Vec::new()
        }
    }

    /// Convert Data Frame to ndarray
    pub fn values(&self) -> Array1<f32> {
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
    pub fn unique(&mut self) -> Column {
        let mut unique_values = HashSet::new();
        self.data.iter().for_each(|e| {
            unique_values.insert(e.clone());
        });
        let mut column = Column::new();
        unique_values.iter().for_each(
            |el| column.data.push(el.clone())
        );
        column
    }

    pub fn len(self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, Clone)]
pub struct DataFrame {
    pub size: usize,
    labels: Vec<String>,
    data: Vec<Column>
}

pub trait DataFrameImpl {
    fn new(size: usize, vec: Option<Vec<Vec<String>>>) -> DataFrame;
    fn push(&mut self, element: Vec<Element>);
    // Get selected column by using label name
    fn by(&mut self, label: &str) -> &mut Column;
    // Get selected columns by using labels name
    fn many(&mut self, labels: Vec<&str>) -> Vec<Column>;
    fn map(&mut self, col: &str, obj: HashMap<&str, u32>) -> DataFrame;
    // One hot encoding - Convert string values to binary value
    fn get_dummies(&mut self, label: &str) -> DataFrame;
    // Concatenate two data frames
    fn concat(&mut self, df: DataFrame) -> DataFrame;
    // Drop column by label from Data Frame
    fn drop(&mut self, label: Vec<&str>) -> Option<DataFrame>;
    // Drop column by position from Data Frame
    fn drop_idx(&mut self, position: usize) -> Option<DataFrame>;
    fn from_vec(vec: Vec<Vec<i32>>, size: usize)-> DataFrame ;
    fn to_vec(&self) -> Vec<Vec<f32>>;
    fn add_labels(&mut self, labels: Vec<String>) -> &DataFrame;
    fn values(&self) -> Vec<Vec<f32>>;
    fn for_each(&self, arr: &mut Vec<Vec<f32>>);
    fn read_csv(file_name: String) -> Result<DataFrame, Box<dyn Error>>;
}

impl Index<usize> for DataFrame {
    type Output = Column;
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for DataFrame {
    fn index_mut(& mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}


impl Index<&str> for DataFrame {
    type Output = Column;
    fn index(&self, i: &str) -> &Self::Output {
        &self.data.iter().find(|p| {
            p.label == i.to_string()
        }).unwrap()
    }
}

impl DataFrameImpl for DataFrame {
    fn new(size: usize, vec: Option<Vec<Vec<String>>>) -> DataFrame {
        DataFrame {
            size,
            labels: Vec::new(),
            data: vec![Column::new(); size]
        }
    }

    fn push(&mut self, element: Vec<Element>) {
        for (i, el) in element.iter().enumerate() {
            self.data[i].push(Element {
                value: el.value.clone()
            });
        }
    }

    /// Get selected column by using label name
    fn by(&mut self, label: &str) -> &mut Column {
        let index = self.labels.clone().iter().position(|
            el| el == label
        ).unwrap();
        &mut self.data[index]
    }

    /// Get selected columns by using labels name
    fn many(&mut self, labels: Vec<&str>) -> Vec<Column> {
        self.data.clone().into_iter().filter(|p| {
            labels.contains(&p.label.as_str())
        }).collect::<Vec<Column>>()
    }

    fn map(&mut self, col: &str, obj: HashMap<&str, u32>) -> DataFrame {
        for el in &mut self.by(col).data {
            for (key, v) in obj.iter() {
                if *key == el.value.as_str() {
                    el.replace(format!("{}", v));
                    break;
                }
            }
        }
        self.to_owned()
    }

    /// One hot encoding - Convert string values to binary value
    fn get_dummies(&mut self, label: &str) -> DataFrame {
        let column = self.by(label);
        let unique_column = column.clone().unique();
        let size = unique_column.clone().len();
        let columns = column.data.iter().map(|el| {
            let mut tmp = vec![0;size];
            let index = unique_column.data.iter().position(
                |it| it.value == el.value
            ).unwrap();
            tmp[index] = 1;
            tmp
        }).collect();
        let mut df = Self::from_vec(columns, size);
        df.add_labels(unique_column.data.iter().map(
            |el| el.value.clone()
        ).collect());
        df
    }

    /// Concatenate two data frames
    fn concat(&mut self, df: DataFrame) -> DataFrame {
        DataFrame {
            size: 0,
            labels: [&self.labels[..], &df.labels[..]].concat(),
            data: [&self.data[..], &df.data[..]].concat()
        }
    }

    /// Drop column by label from Data Frame
    fn drop(&mut self, labels: Vec<&str>) -> Option<DataFrame> {
        for label in labels {
            let position = self.labels.clone().iter().position(
                |el| el == label
            ).unwrap();
            self.labels.remove(position);
            self.data.remove(position);
        }
        let a = self.to_owned();
        Some(a)
    }

    /// Drop column by position from Data Frame
    fn drop_idx(&mut self, position: usize) -> Option<DataFrame> {
        if self.labels.len() < position {
            return None;
        }
        self.labels.remove(position);
        self.data.remove(position);
        Some(self.to_owned())
    }

    fn from_vec(vec: Vec<Vec<i32>>, size: usize)-> DataFrame {
        let mut df = DataFrame::new(size, None);
        for (_, columns) in vec.iter().enumerate() {
            for (index, value) in columns.iter().enumerate() {
                df.data[index].push(Element { value: format!("{}", value)});
            }
        }
        df
    }

    fn to_vec(&self) -> Vec<Vec<f32>> {
        let mut arr = vec![vec![0.0;self.labels.len()];self.data[0].data.len()];
        self.for_each(&mut arr);
        arr
    }

    fn add_labels(&mut self, labels: Vec<String>) -> &DataFrame {
        self.labels = labels.clone();
        for (index, label) in labels.iter().enumerate() {
            self.data[index].label = label.clone();
        }
        self
    }

    fn values(&self) -> Vec<Vec<f32>> {
        let mut arr = vec![vec![0.0;self.labels.len()];self.data[0].data.len()];
        // let mut arr = Array::default((self.labels.len(), self.data.len()));
        self.for_each(&mut arr);
        arr
    }

    fn for_each(&self, arr: &mut Vec<Vec<f32>>) {
        for (col, column) in self.data.iter().enumerate() {
            for (row, el) in column.data.iter().enumerate() {
                match el.value.parse() {
                    Ok(num) => {
                        arr[row][col] = num;
                    },
                    Err(_) => {
                        arr[row][col] = 0.0;
                    },
                }
            }
        }
    }

    fn read_csv(file_name: String) -> Result<DataFrame, Box<dyn Error>> {
        let file = File::open(file_name)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut df = DataFrame::new(rdr.headers().unwrap().len(), None);
        let mut vec = Vec::new();
        for header in rdr.headers() {
            for el in header.iter() {
                vec.push(el.to_string())
            }
        }
        df.add_labels(vec);
        for result in rdr.records(){
            let mut row =Vec::new();
            let record = result?;
            for el in record.iter() {
                row.push(Element { value: el.to_string() });
            }
            df.push(row);
        }
        Ok(df)
    }
}