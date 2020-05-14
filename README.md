# Duck ML
### Duck is implementation in Rust Machine Learning Library.
In this moment we start creating DataFrame.
We want to use ndarray as some numpy alternative and nalgebra as mathematic part of coding.

#### Contributors:
    Mateusz Kubaszek - mkubasz
    Paweł Walus - Losseheil
#### Ducky's Structures:
    type DataFrame -> Data Enginnering
        eg: 
        DataFrame::new(
                    vec![
                        row![0.4, 0.7, "poster", true, 1],
                        row![3.0, 4.7, "table", true, 1],
                        row![3.0, 4.7, "book", true, 1],
                    ],
                    vec!["A", "B", "C", "D", "E"]
                );
    type DataFrameGroupBy -> Hashmap
    type Series -> Vectorize structure
        eg:
        Series::new(
            row![0.4, 0.7, "poster", true, 1],
            String::from("A")
        )
    type Cell -> One element in Data Frame
        - Number
        - Float
        - Float64
        - Bool
        - Text
    type Matrix -> Algebraic matrix

#### Ducky's Operations:
    fn push(&mut self, element: Vec<Cell>);
    
    fn series(&mut self, index: usize) -> &mut Series<Cell>;
    
    fn to_rows(&self) -> Option<Vec<Vec<Cell>>>;

    fn by(&mut self, label: &str) -> Option<&mut Series<Cell>>;
    
    fn many(&mut self, labels: Vec<&str>) -> Vec<Series<Cell>>;
    
    fn map(&mut self, col: &str, obj: HashMap<&str, u32>) -> DataFrame;
    
    fn concat(&mut self, df: DataFrame) -> DataFrame;
    
    fn drop(&mut self, label: Vec<&str>) -> Option<DataFrame>;
    
    fn drop_idx(&mut self, position: usize) -> Option<DataFrame>;
    
    fn contains(self, label: &str) -> bool;
    
    fn join(&mut self, df: DataFrame);
    
    fn group_by(&mut self, label: &str) -> Option<DataFrameGroupBy>;
    
    fn sort(&mut self, label: &str) -> Option<DataFrame>;
    
    fn read_csv(file_name: String) -> Result<DataFrame, Box<dyn Error>>;
#### Ducky's Science:
    fn get_dummies(&mut self, label: &str) -> DataFrame;
   
    fn from_vec<T>(vec: Vec<Vec<T>>, labels: Vec<&str>) -> DataFrame where T: PrimInt + Signed + Product;
   
    fn to_matrix(&self) -> Option<Matrix>;
#### Ducky's Math:
    fn sigmoid(x: f64) -> f64;
    
    fn minmax_item<T: Into<f64> + Copy>(min_max: (f64, f64), item: T) -> (f64, f64);
    
    fn minmax<T: Into<f64> + Copy>(series: Vec<T>) -> Option<(f64, f64)>;

    fn series_minmax(series: Series<Cell>) -> (f64, f64);

#### Development milestone:

In first step:
 - Algorithms: Evaluation Methods, Evaluation Metrics, Baseline Models.

In second step:
 - Algorithms: (Simple/Multivariate) Linear Regression, Logistic Regression, Perceptron.

In third step:
 - Algorithms: Decision Trees, Regression Trees, Naive Bayes, k-nearest, Backpropagation.
