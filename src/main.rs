use std::io;
use std::cmp::Ordering;
use rand::Rng;
use na::DMatrix;

// first piece in weight vector is bias
// first piece in input vector is 1f64
// bias is a negative
//activation function such that y = 1 if A(i*w) > 0 else 0

fn main() {
	let bias = 0f64;
	let learning_rate = 0f64;
	let epochs = 50;
	let test_csv = String::from("../mnist_test.csv");
	let train_csv = String::from("../mnist_train.csv");
	let test_set = preprocess(load_data(test_csv));
	let train_set = preprocess(load_data(train_csv));
	let mut weights = init_weights();
	//calculate initial accuracy on training and test data
	for i in 0..epochs {
		//train
		//calculate accuracy on training and test data
	}
	//show plots and confusion matrix
}

fn load_data(file:String) -> DMatrix{}
fn preprocess(data:DMatrix) {}
fn init_weights() -> DMatrix{}
fn activation() {}
// the loss function used here is 1/2 * (expected y - actual y)^2
// where y is the result of the activation function
fn train() {}
fn loss() {}