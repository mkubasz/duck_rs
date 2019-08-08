use std::fs::File;
use std::error::Error;
use std::env;
use std::collections::HashSet;
use std::borrow::{Borrow, BorrowMut};
use ndarray::Data;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Element {
    value: String
}

impl Element {
    fn push() {
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub data: Vec<Element>
}

impl Column {
    fn new() -> Column {
        Column {
            data: Vec::new()
        }
    }

    fn push(&mut self, element: Element) {
        self.data.push(element);
    }

    pub fn unique(&mut self) -> Column {
        let mut unique_values = HashSet::new();
        self.data.iter().for_each(|e| {
            unique_values.insert(Element {
                value: e.value.clone()
            });
        });
        let mut column = Column::new();
        unique_values.iter().for_each(|el| column.data.push(Element { value: el.value.clone()}) );
        column
    }

    pub fn len(self) -> usize {
        self.data.len()
    }
}

#[derive(Debug, Clone)]
pub struct DataFrame {
    label: Vec<String>,
    data: Vec<Column>
}

impl DataFrame {
    pub fn new(size: usize) -> DataFrame {
        DataFrame {
            label: Vec::new(),
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
        let index = self.label.clone().iter().position(|el| el == label ).unwrap();
        &mut self.data[index]
    }

    pub fn get_dummies(&mut self, label: &str) -> DataFrame {
        let column = self.by(label);
        let unique_column = column.unique();
        let size = unique_column.clone().len();
        let columns = column.data.iter().map(|el| {
            let mut tmp = vec![0;size];
            let index = unique_column.data.iter().position(|it| it == el).unwrap();
            tmp[index] = 1;
            tmp
        }).collect();
        let mut df = Self::from_vec(columns, size);
        df.add_labels(unique_column.data.iter().map(|el| el.value.clone()).collect());
        df
    }
    pub fn concat(&mut self, df: DataFrame) -> DataFrame {
        DataFrame{ label: [&self.label[..], &df.label[..]].concat(), data: [&self.data[..], &df.data[..]].concat() }
    }

    pub fn drop(&mut self, label: &str) -> DataFrame {
        let position = self.label.clone().iter().position(|el| el == label ).unwrap();
        self.label.remove(position);
        self.data.remove(position);
        self.to_owned()
    }

    pub fn from_vec(vec: Vec<Vec<i32>>, size: usize)-> DataFrame {
        let mut df = DataFrame::new(size);
        for (_, columns) in vec.iter().enumerate() {
            for (index, value) in columns.iter().enumerate() {
                df.data[index].push(Element {value: format!("{}", value)});
            }
        }
        df
    }

    pub fn add_labels(&mut self, labels: Vec<String>) -> &DataFrame {
        self.label = labels.clone();
        self
    }

    pub fn read_csv(file_name: String) -> Result<DataFrame, Box<dyn Error>> {
        let path = env::current_dir()?;
        println!("{:?}", path);
        let mut file = File::open(file_name)?;

        let mut rdr = csv::Reader::from_reader(file);
        let mut df = DataFrame::new(rdr.headers().unwrap().len());
        for header in rdr.headers() {
            for el in header.iter() {
                df.label.push(el.to_string());
            }
        }
        for result in rdr.records(){
            let mut row =Vec::new();
            let record = result?;
            for el in record.iter() {
                row.push(Element {value: el.to_string() });
            }
            df.push(row);
        }
        print!("{:?}", df);
        Ok(df)
    }
}