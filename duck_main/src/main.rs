use rustlearn::prelude::*;
use rustlearn::linear_models::sgdclassifier::Hyperparameters;
use duck_main::DataFrame;
use rustlearn::trees::decision_tree;

fn main() {
    let mut df = DataFrame::read_csv(format!("src/Startups.csv")).unwrap();
    let y = df.by("Profit").clone();
    let mut X = df.idx_drop(4);
    let dummies = X.get_dummies("State");
    X = X.concat(dummies).drop("State");
    let X = &Array::from(&X.values());
    let y = &Array::from(y.values().to_vec());
    let mut tree_params = decision_tree::Hyperparameters::new(X.cols());
    tree_params.min_samples_split(10)
        .max_features(4);

    let mut model = Hyperparameters::new(6)
        .one_vs_rest();

    model.fit(X, y).unwrap();
    model.predict(X).unwrap();
}
