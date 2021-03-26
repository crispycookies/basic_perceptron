use crate::perceptron::Perceptron;

mod node;
mod perceptron;
mod util;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt::Debug;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// Accessed 20.02.2021, 21:30-22:31
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_train_data<T>(f_name: String, size: usize) -> Vec<Vec<T>>
    where T: std::str::FromStr, <T as std::str::FromStr>::Err: Debug
{
    let mut x = std::vec::Vec::new();
    if let Ok(lines) = read_lines(f_name.as_str()) {
        for line in lines {
            match line {
                Ok(ip) => {
                    let tokens: Vec<&str> = ip.split(", ").collect();
                    if tokens.len() != size {
                        panic!("Invalid Data Format");
                    }
                    let mut v = std::vec::Vec::new();

                    for i in 0..size {
                        v.push(tokens.get(i).unwrap().to_string().parse::<T>().unwrap());
                    }

                    x.push(v);
                }
                _ => {
                    panic!("Line could not be read");
                }
            }
        }
    } else {
        panic!("File could not be found or opened");
    };
    return x;
}

pub fn get_training_data_labels<T>(f_name: String, label_a: String, label_b: String, lower: T, upper: T) -> Vec<T>
    where T: std::str::FromStr + Copy, <T as std::str::FromStr>::Err: Debug
{
    let mut y = std::vec::Vec::new();
    if let Ok(lines) = read_lines(f_name.as_str()) {
        for line in lines {
            match line {
                Ok(ip) => {
                    // Mapper
                    if ip.contains(label_a.as_str()) {
                        y.push(lower);
                    } else if ip.contains(label_b.as_str()) {
                        y.push(upper);
                    } else {
                        panic!("File could not be parsed!");
                    }
                }
                _ => {
                    panic!("Line could not be read");
                }
            }
        }
    } else {
        panic!("File could not be found or opened");
    };
    return y;
}

fn predict<T>
(def_weight: T, upper: T, lower: T, zero: T, size: usize, prediction_data: String, train_data: String, train_label_data: String, label_a: String, label_b: String, epochs: u64, eta: T) -> Vec<T>
    where T: Copy + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::cmp::PartialOrd + std::str::FromStr,
          <T as std::str::FromStr>::Err: Debug

{
    let mut perceptron = Perceptron::new(def_weight, upper, lower, zero, size);
    let x = get_train_data::<T>(train_data, size - 1);
    let d = get_train_data::<T>(prediction_data, size - 1);
    let y = get_training_data_labels::<T>(train_label_data, label_a, label_b, lower, upper);
    perceptron.train(x.clone(), y.clone(), epochs, eta);

    let mut r = Vec::new();
    for i in d {
        r.push(perceptron.predict(i));
    }
    return r;
}


fn verify<T>(f_name: String, predicted: Vec<T>, label_a: String, label_b: String, upper: T, lower: T) -> (f64, i32, usize)
    where T: std::str::FromStr + Copy + std::cmp::PartialEq, <T as std::str::FromStr>::Err: Debug
{
    let y = get_training_data_labels::<T>(f_name, label_a, label_b, lower, upper);
    let mut counter = 0;

    for i in 0..y.len() {
        if predicted.get(i).unwrap() != y.get(i).unwrap() {
            counter = counter + 1;
        }
    }

    let accuracy = 1. - (counter as f64 / predicted.len() as f64);
    return (accuracy, counter, predicted.len());
}

fn parse_and_run<T>
(args: Vec<String>)
    where T: Copy + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::cmp::PartialOrd + std::str::FromStr + std::fmt::Display, <T as std::str::FromStr>::Err: Debug
{
    let def_weight = args.get(1).unwrap().parse::<T>().unwrap();
    let upper = args.get(2).unwrap().parse::<T>().unwrap();
    let lower = args.get(3).unwrap().parse::<T>().unwrap();
    let zero = args.get(4).unwrap().parse::<T>().unwrap();
    let size = args.get(5).unwrap().parse::<usize>().unwrap();
    let train = args.get(6).unwrap().clone();
    let label = args.get(7).unwrap().clone();
    let label_a = args.get(8).unwrap().clone();
    let label_b = args.get(9).unwrap().clone();
    let epochs = args.get(10).unwrap().clone().parse::<u64>().unwrap();
    let eta = args.get(11).unwrap().clone().parse::<T>().unwrap();
    let prediction = args.get(12).unwrap().clone();
    let prediction_label = args.get(13).unwrap().clone();


    let r = predict(def_weight, upper, lower, zero, size, prediction, train, label, label_a.clone(), label_b.clone(), epochs, eta);
    let a = verify::<T>(prediction_label, r, label_a.clone(), label_b.clone(), upper, lower);

    println!("Accuracy {}", a.0);
    println!("Errors {}", a.1);
    println!("Size {}", a.2);
    println!("Epochs {}", epochs);
    println!("ETA {}", eta);
}




fn main() {
    let args: Vec<String> = std::env::args().collect();



    match args.get(0).unwrap().as_str() {
        "f32" => {
            parse_and_run::<f32>(args.clone());
        }
        "f64" => {
            parse_and_run::<f64>(args.clone());
        }
        "u8" => {
            parse_and_run::<u8>(args.clone());
        }
        "u16" => {
            parse_and_run::<u16>(args.clone());
        }
        "u32" => {
            parse_and_run::<u32>(args.clone());
        }
        "u64" => {
            parse_and_run::<u64>(args.clone());
        }
        "u128" => {
            parse_and_run::<u128>(args.clone());
        }
        &_ => {
            panic!("No viable Type Provided at <1>")
        }
    }
}
