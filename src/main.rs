extern crate nalgebra;

use std::cmp::Ordering;
use rand::Rng;
use std::error::Error;
use std::io;
use std::process;
use nalgebra::{DMatrix,DVector};

// first piece in weight vector is bias
// first piece in input vector is 1f64
// bias is a negative
//activation function such that y = 1 if A(i*w) > 0 else 0

struct Data {
	target:DMatrix<f64>,
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
	let mut weights = init_weights(10, 400);
	let mut test_accuracy = vec![0f64; epochs+1];
	let mut train_accuracy = vec![0f64; epochs+1];
	let mut conf_matrix: DMatrix<f64> = DMatrix::from_element(10,10,0);

	//calculate initial accuracy on training and test data
	for i in 0..epochs {
		//let test_act = activation(result(test_set.data, weights));
		//let train_act = activation(result(train_set.data, weights));
		let test_y = activation(test_set.data, weights);
		let train_y = activation(train_set.data, weights);

		train(test_set, weights, test_y, learning_rate);
		test_accuracy[i] = accuracy(test_set.target, test_y);
		train_accuracy[i] = accuracy(train_set.target, train_y);
		conf_matrix = 
			test_set.target.transpose()*test_y + 
			train_set.target.transpose()*train_y;
	}

	//show plots and confusion matrix
	show_graph();
	show_conf_matrix(conf_matrix);
	// let a: f64 = 1.0/255.0;
	// let b: DMatrix<f64> = DMatrix::from_vec(4, 3, vec![1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
	// let result = a*b;
	// println!("{}", result);
}

fn load_data(file:String) -> Data {}

fn preprocess(data:Data) -> Data {
	let scalar: f64 = 1.0/255.0;
	let processed = scalar * data.data;
	return Data {data:processed,target:data.target};
}

fn init_weights(width:usize, height:usize) -> DMatrix<f64> {
	let length = width * height;
	let mut vec: Vec<f64> = Vec::with_capacity(length);
	for _ in 0..vec.capacity() {
		vec.push(rand::thread_rng().gen_range(-0.5,0.5));
	}
	return DMatrix::from_vec(width, height, vec);
}

//fn result(data:DMatrix<f64>, weigths:DMatrix<f64>) -> DMatrix<f64> {}
//fn activation(results:DMatrix<f64>) -> DMatrix<u8> {}
fn activation(x:DMatrix<f64>, w:DMatrix<f64>) -> DMatrix<f64> {
	let result = x*w;
	return DMatrix::from_fn(result.ncols(), result.nrows(), |r,c| if result[(r,c)] > 0.0f64 {1.0} else {0.0});
}

//x is Xx400
//w is 400x10
//t is Xx10
//y is Xx10
//dw is 400x10
fn train(data:Data, w:DMatrix<f64>, y:DMatrix<f64>, learning_rate:f64) {
	let t = data.target;
	let x = data.data;
	let delta = learning_rate* x.transpose() * (t - y);
	w = w + delta;
}

fn accuracy(conf_matrix: DMatrix<f64>) -> DVector<f64> {
	// let identity = DVector::from_element(conf_matrix.nrows(), 1.0f64);
	// let a = conf_matrix.row(0);
	// let b = conf_matrix[(0,0)];
	// let c = DVector::from_fn(conf_matrix.nrows(), |1,i| {conf_matrix.row(0);});
	// return DVector::from_fn(conf_matrix.nrows(), |i| conf_matrix[(i,i)]/(DVector::from_slice(conf_matrix.row(i)).dot(identity)));
	let mut vec: Vec<f64> = Vec::with_capacity(conf_matrix.nrows());
	for i in 0..vec.capacity() {
		let total = 0.0;
		for j in 0..conf_matrix.ncols() {
			total += conf_matrix[(i,j)];
		}
		if total == 0.0 {
			continue;
		}
		vec.push(conf_matrix[(i,i)]/total);
	}
	return DVector::from_fn(vec.capacity(), |i, 1| {vec[i]});
}

// the loss function used here is 1/2 * (expected y - actual y)^2
// where y is the result of the activation function
//fn loss(activation:u8) -> f64 {}

fn show_graph() {}
fn show_conf_matrix(conf_matrix:DMatrix<f64>) {}