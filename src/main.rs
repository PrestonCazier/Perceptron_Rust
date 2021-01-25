use std::cmp::Ordering;
use rand::Rng;
use nalgebra::DMatrix;
use nalgebra::DVector;

// first piece in weight vector is bias
// first piece in input vector is 1f64
// bias is a negative
//activation function such that y = 1 if A(i*w) > 0 else 0

struct Data {
	target:DMatrix<u8>,
	data:DMatrix<f64>
}

fn main() {
	let bias = 0f64;
	let learning_rate = 0f64;
	let epochs = 50;
	let test_csv = String::from("../mnist_test.csv");
	let train_csv = String::from("../mnist_train.csv");
	let test_set: Data = preprocess(load_data(test_csv));
	let train_set: Data = preprocess(load_data(train_csv));
	let mut weights = init_weights();
	let mut test_accuracy = vec![0f64; epochs+1];
	let mut train_accuracy = vec![0f64; epochs+1];
	let mut conf_matrix: DMatrix<u8> = DMatrix::from_element(10,10,0);

	//calculate initial accuracy on training and test data
	for i in 0..epochs {
		//let test_act = activation(result(test_set.data, weights));
		//let train_act = activation(result(train_set.data, weights));
		let test_y = activation(test_set.data, weights);
		let train_y = activation(train_set.data, weights);

		train(test_set, weights, test_y);
		test_accuracy[i] = accuracy(test_set.target, test_y);
		train_accuracy[i] = accuracy(train_set.target, train_y);
		conf_matrix = 
			test_set.target.transpose()*test_y + 
			train_set.target.transpose()*train_y;
	}

	//show plots and confusion matrix
	show_graph();
	show_conf_matrix(conf_matrix);
}

fn load_data(file:String) -> Data {}
fn preprocess(data:Data) -> Data {}
fn init_weights() -> DMatrix<f64> {}

//fn result(data:DMatrix<f64>, weigths:DMatrix<f64>) -> DMatrix<f64> {}
//fn activation(results:DMatrix<f64>) -> DMatrix<u8> {}
fn activation(data:DMatrix<f64>, w:DMatrix<f64>) -> DMatrix<u8> {}

fn train(data:Data, w:DMatrix<f64>, y:DMatrix<u8>) {}
fn accuracy(t: DMatrix<u8>, y: DMatrix<u8>) -> f64 {}

// the loss function used here is 1/2 * (expected y - actual y)^2
// where y is the result of the activation function
//fn loss(activation:u8) -> f64 {}

fn show_graph() {}
fn show_conf_matrix(conf_matrix:DMatrix<u8>) {}