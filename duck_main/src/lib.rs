pub mod duck {
   // use std::collections::HashMap;

//    pub struct DataFrame {
//        pub data: HashMap<&'static str, Vec<&'static str>>,
//    }

    pub struct NDArray {
        pub data: Vec<u32>,
        pub size: usize,
        pub d_type: &'static str,
        pub shape: usize,
    }

    impl NDArray {
        pub fn new() -> Self {
            NDArray {
                data: Vec::new(),
                size: 0,
                d_type: "int64",
                shape: 0}
        }
    }

//    pub struct Duck {
//        nd_array: NDArray
//    }

    pub struct Column {
        pub data: NDArray,
    }

    impl Column {
        pub fn new(data: &[u32]) -> Self {
            let mut c =  Column{data: NDArray::new()};
            c.column(data);
            c
        }
        pub fn column(&mut self, arr: &[u32])
        {
            let mut count: usize = 0;
            let shape: usize = 1;

            for (_, it) in arr.iter().enumerate() {
                self.data.data.push(*it);
                count += 1;
            }


            self.data.size = count;
            self.data.shape = shape;
        }
    }

    pub struct Matrix {
        pub nd_array: Vec<Column>,
        pub size: usize,
    }

    impl Matrix {
        pub fn new(data: Vec<Column>) -> Self {
            let mut size: usize = 0;
            for i in data.iter() {
                size += i.data.data.len();
            }

            Matrix {
                nd_array: data,
                size,
            }
        }
        pub fn data(&self) -> Vec<Vec<u32>> {
           let mut vec: Vec<Vec<u32>> = Vec::new();
            for i in self.nd_array.iter() {
                vec.push(i.data.data.clone())
            }
            vec
        }
        pub fn size(&self) -> usize {
            self.size
        }
        pub fn shape(&self) -> u32 {
            1
        }
        pub fn d_type(&self) -> &'static str {
            "u32"
        }
    }
    pub struct Vector {
        pub data: Column,
    }

    impl Vector {
        pub fn new(data: &[u32]) -> Self {
            Vector {
                data: Column::new(data)
            }
        }

        pub fn data(&self) -> &Vec<u32> {
            &self.data.data.data
        }
        pub fn size(&self) -> usize {
            self.data.data.size
        }
        pub fn shape(&self) -> usize {
            self.data.data.shape
        }
        pub fn d_type(&self) -> &'static str {
            self.data.data.d_type
        }
    }

//    impl Duck {
//        pub fn new() -> Self {
//            //Duck { }
//        }
//
//
//
//        pub fn array(&mut self, arr: Vec<Row>) -> &NDArray
//        {
//            let mut count: usize = 0;
//            let shape: usize = 1;
//
//            for (_, it) in arr.iter().enumerate() {
//                self.nd_array.data.push((**it)[0]);
//                count += 1;
//            }
//
//
//            self.nd_array.size = count;
//            self.nd_array.shape = shape;
//            &self.nd_array
//        }
//    }

}