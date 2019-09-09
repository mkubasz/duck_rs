# Duck ML
### Duck is implementation in Rust Machine Learning Library.
In this moment we start creating DataFrame.
We want to use ndarray as some numpy alternative and nalgebra as mathematic part of coding.

#### Ducky's Operations:
    fn push(&mut self, element: Vec<Cell>);
    fn series(&mut self, index: usize) -> &mut Series<Cell>;
    fn by(&mut self, label: &str) -> Option<&mut Series<Cell>>;
    fn many(&mut self, labels: Vec<&str>) -> Vec<Series<Cell>>;
    fn map(&mut self, col: &str, obj: HashMap<&str, u32>) -> DataFrame;
    fn concat(&mut self, df: DataFrame) -> DataFrame;
    fn drop(&mut self, label: Vec<&str>) -> Option<DataFrame>;
    fn drop_idx(&mut self, position: usize) -> Option<DataFrame>;
    fn contains(self, label: &str) -> bool;
    fn read_csv(file_name: String) -> Result<DataFrame, Box<dyn Error>>;
#### Ducky's Science:
    fn get_dummies(&mut self, label: &str) -> DataFrame;
    fn from_vec<T>(vec: Vec<Vec<T>>, labels: Vec<&str>) -> DataFrame where T: PrimInt + Signed + Product;
#### Ducky's Math:
  - minmax

In first step:
Algorithms: Evaluation Methods, Evaluation Metrics, Baseline Models.

In second step:
Algorithms: (Simple/Multivariate) Linear Regression, Logistic Regression, Perceptron.

In third step:
Algorithms: Decision Trees, Regression Trees, Naive Bayes, k-nearest, Backpropagation.
