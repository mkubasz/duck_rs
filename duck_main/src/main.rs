use rustlearn::prelude::*;
use rustlearn::linear_models::sgdclassifier::Hyperparameters;
use rustlearn::datasets::iris;
use duck_main::{DataFrame, Element};

fn main() {
    let mut df = DataFrame::read_csv(format!("src/Startups.csv")).unwrap();
    let dummies = df.get_dummies("State");
    let mut new_df = df.concat(dummies).drop("State");
    let mut model = Hyperparameters::new(4)
        .learning_rate(1.0)
        .l2_penalty(0.5)
        .l1_penalty(0.0)
        .one_vs_rest();

    model.fit(&X, &y).unwrap();

    let prediction = model.predict(&X).unwrap();
    println!("{:?}", prediction);
}
