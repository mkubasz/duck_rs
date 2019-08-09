use std::fs::File;
use std::error::Error;
use std::collections::HashSet;
use std::ops::{Index, IndexMut};
use ndarray::prelude::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Element {
    value: String
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
    labels: Vec<String>,
    data: Vec<Column>
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

impl DataFrame {
    pub fn new(size: usize) -> DataFrame {
        DataFrame {
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

    pub fn by(&mut self, label: &str) -> &mut Column {
        let index = self.labels.clone().iter().position(|
            el| el == label
        ).unwrap();
        &mut self.data[index]
    }

    pub fn many(&mut self, labels: Vec<&str>) -> Vec<Column> {
        self.data.clone().into_iter().filter(|p| {
            labels.contains(&p.label.as_str())
        }).collect::<Vec<Column>>()
    }

    pub fn get_dummies(&mut self, label: &str) -> DataFrame {
        let column = self.by(label);
        let unique_column = column.unique();
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
    pub fn concat(&mut self, df: DataFrame) -> DataFrame {
        DataFrame {
            labels: [&self.labels[..], &df.labels[..]].concat(),
            data: [&self.data[..], &df.data[..]].concat()
        }
    }

    pub fn drop(&mut self, label: &str) -> DataFrame {
        let position = self.labels.clone().iter().position(
            |el| el == label
        ).unwrap();
        self.labels.remove(position);
        self.data.remove(position);
        self.to_owned()
    }

    pub fn drop_idx(&mut self, position: usize) -> Option<DataFrame> {
        if self.labels.len() < position {
            return None;
        }
        self.labels.remove(position);
        self.data.remove(position);
        Some(self.to_owned())
    }

    pub fn from_vec(vec: Vec<Vec<i32>>, size: usize)-> DataFrame {
        let mut df = DataFrame::new(size);
        for (_, columns) in vec.iter().enumerate() {
            for (index, value) in columns.iter().enumerate() {
                df.data[index].push(Element { value: format!("{}", value)});
            }
        }
        df
    }

    pub fn add_labels(&mut self, labels: Vec<String>) -> &DataFrame {
        self.labels = labels.clone();
        for (index, label) in labels.iter().enumerate() {
            self.data[index].label = label.clone();
        }
        self
    }

    pub fn values(&self) -> Vec<Vec<f32>> {
        //let mut arr = Array::default((self.labels.len(), self.data.len()));
        let mut arr = vec![vec![0.0;self.labels.len()];self.data[0].data.len()];
        for (i, columns) in self.data.iter().enumerate() {
            for (j, el) in columns.data.iter().enumerate() {
                arr[j][i] = el.value.parse().unwrap();
            }
        }
        arr
    }

    pub fn read_csv(file_name: String) -> Result<DataFrame, Box<dyn Error>> {
        let file = File::open(file_name)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut df = DataFrame::new(rdr.headers().unwrap().len());
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