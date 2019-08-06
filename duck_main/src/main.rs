use rustlearn::prelude::*;
use rustlearn::linear_models::sgdclassifier::Hyperparameters;
use rustlearn::datasets::iris;
use duck_main::read;

fn main() {
    let (X, y) = iris::load_data();
    read();
    let mut model = Hyperparameters::new(4)
        .learning_rate(1.0)
        .l2_penalty(0.5)
        .l1_penalty(0.0)
        .one_vs_rest();

    model.fit(&X, &y).unwrap();

    let prediction = model.predict(&X).unwrap();
    //println!("{:?}", prediction);
}
