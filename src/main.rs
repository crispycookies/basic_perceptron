use crate::perceptron::Perceptron;

mod node;
mod perceptron;
mod util;

use std::io::Write;
use std::fmt::Debug;
use crate::util::Util;
use std::fs::File;
use crate::node::unchecked_ops;

fn verify<T>(actual: Vec<T>, predicted: Vec<T>) -> (f64, i32, usize)
    where T: std::str::FromStr + Copy + std::cmp::PartialEq, <T as std::str::FromStr>::Err: Debug
{
    let mut counter = 0;

    for i in 0..actual.len() {
        if predicted.get(i).unwrap() != actual.get(i).unwrap() {
            counter = counter + 1;
        }
    }

    let accuracy = 1. - (counter as f64 / predicted.len() as f64);
    return (accuracy, counter, predicted.len());
}

fn store_actual<T>(data: Vec<Vec<T>>, actual: Vec<T>, predicted: Vec<T>, name: String, util: Util<T>)
    where T: std::str::FromStr + Copy + std::cmp::PartialEq + std::fmt::Display + std::ops::Add<Output=T>, <T as std::str::FromStr>::Err: Debug
{
    let mut file = File::create(name + ".dat.csv").unwrap();

    for _ in 0..data.get(0).unwrap().len() {
        file.write(b"property,").unwrap();
    }
    file.write(b"actual,").unwrap();
    file.write(b"predicted\n").unwrap();

    for i in 0..data.len() {
        let c = data.get(i).unwrap();
        for d in c {
            let buff = d.to_string() + ",";
            file.write(buff.as_bytes()).unwrap();
        }
        let buff_actual = util.resolve(*actual.get(i).unwrap()) + ",";
        let buff_predicted = util.resolve(*predicted.get(i).unwrap()) + "\n";
        file.write(buff_actual.as_bytes()).unwrap();
        file.write(buff_predicted.as_bytes()).unwrap();
    }
}

fn store_result<T>(accuracy: f64, errors: i32, count: usize, epochs: u64, eta: T, name: String)
    where T: std::str::FromStr + Copy + std::cmp::PartialEq + std::fmt::Display, <T as std::str::FromStr>::Err: Debug
{
    let mut file = File::create(name + ".res.csv").unwrap();

    file.write(b"Accuracy,Errors,Size,Epochs,ETA\n").unwrap();
    file.write(accuracy.to_string().as_bytes()).unwrap();
    file.write(b",").unwrap();
    file.write(errors.to_string().as_bytes()).unwrap();
    file.write(b",").unwrap();
    file.write(count.to_string().as_bytes()).unwrap();
    file.write(b",").unwrap();
    file.write(epochs.to_string().as_bytes()).unwrap();
    file.write(b",").unwrap();
    file.write(eta.to_string().as_bytes()).unwrap();
    file.write(b"\n").unwrap();


    println!("Accuracy {}", accuracy);
    println!("Errors {}", errors);
    println!("Size {}", count);
    println!("Epochs {}", epochs);
    println!("ETA {}", eta);
}

fn parse_and_run<T>
(args: Vec<String>)
    where T: Copy + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::cmp::PartialOrd + std::str::FromStr + std::fmt::Display + util::convert::Convert + unchecked_ops::UncheckedOps, <T as std::str::FromStr>::Err: Debug
{
    let def_weight = args.get(1).unwrap().parse::<T>().unwrap();
    let upper = args.get(2).unwrap().parse::<T>().unwrap();
    let lower = args.get(3).unwrap().parse::<T>().unwrap();
    let zero = args.get(4).unwrap().parse::<T>().unwrap();
    let offset = args.get(5).unwrap().parse::<T>().unwrap();
    let size = args.get(6).unwrap().parse::<usize>().unwrap();
    let data = args.get(7).unwrap().clone();
    let o_file = args.get(8).unwrap().clone();
    let training_size = args.get(9).unwrap().parse::<usize>().unwrap();
    let epochs = args.get(10).unwrap().clone().parse::<u64>().unwrap();
    let eta = args.get(11).unwrap().clone().parse::<T>().unwrap();


    let mut util = Util::new(size, data, offset, lower);

    match args.get(12) {
        None => {
            let _ = util.read_file();
        }
        Some(e) => {
            let _ = util.read_file_with_scaler(e.parse::<f64>().unwrap());
        }
    }

    let _ = util.generate_map();
    let _ = util.map_file();
    let _ = util.shuffle();
    let (training, validation) = util.cut(training_size);
    let (training_data, training_labels) = util.split_label_from_data(training);
    let (validation_data, validation_labels) = util.split_label_from_data(validation);

    let mut perceptron = Perceptron::new(def_weight, upper, lower, zero, size);
    let _ = perceptron.train(training_data, training_labels, epochs, eta);

    let mut r = Vec::new();
    for i in validation_data.clone() {
        r.push(perceptron.predict(i));
    }

    let accuracy = verify::<T>(validation_labels.clone(), r.clone());
    let _ = store_actual(validation_data.clone(), validation_labels.clone(), r.clone(), o_file.clone(), util);
    let _ = store_result(accuracy.0, accuracy.1, accuracy.2, epochs, eta, o_file.clone());
}


fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    match args.get(0).unwrap().as_str() {
        "f32" => {
            parse_and_run::<f32>(args.clone());
        }
        "f64" => {
            parse_and_run::<f64>(args.clone());
        }
        "i8" => {
            parse_and_run::<i8>(args.clone());
        }
        "i16" => {
            parse_and_run::<i16>(args.clone());
        }
        "i32" => {
            parse_and_run::<i32>(args.clone());
        }
        "i64" => {
            parse_and_run::<i64>(args.clone());
        }
        "u128" => {
            parse_and_run::<i128>(args.clone());
        }
        &_ => {
            panic!("No viable Type Provided at <1>")
        }
    }
}
