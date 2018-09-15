pub mod duck {
    use std::collections::HashMap;

    pub struct DataFrame {
        pub data: HashMap<&'static str, Vec<&'static str>>,
    }

    pub struct NDArray {
        pub data: Vec<i64>,
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

    pub struct Duck {
        nd_array: NDArray
    }

    impl Duck {
        pub fn new() -> Self {
            Duck { nd_array: NDArray::new() }
        }

        pub fn array(&mut self, arr: &[i64]) -> &NDArray {
            let mut count: usize = 0;
            let shape: usize = 1;
            for (_, it) in arr.iter().enumerate() {
                self.nd_array.data.push(*it);
                count += 1;
            }
            self.nd_array.size = count;
            self.nd_array.shape = shape;
            &self.nd_array
        }
    }
}