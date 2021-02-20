use crate::perceptron::Perceptron;

mod node;
mod perceptron;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// Accessed 20.02.2021, 21:30-22:31
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_iris_x() -> Vec<Vec<f64>> {
    let mut x = std::vec::Vec::new();
    if let Ok(lines) = read_lines("data/x.csv") {
        for line in lines {
            match line {
                Ok(ip) => {
                   let tokens : Vec<&str> = ip.split(", ").collect();
                    if tokens.len() != 4 {
                        panic!("Invalid Data Format");
                    }
                    let mut v = std::vec::Vec::new();
                    v.push(tokens.get(0).unwrap().to_string().parse::<f64>().unwrap());
                    v.push(tokens.get(1).unwrap().to_string().parse::<f64>().unwrap());
                    v.push(tokens.get(2).unwrap().to_string().parse::<f64>().unwrap());
                    v.push(tokens.get(3).unwrap().to_string().parse::<f64>().unwrap());
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

pub fn get_iris_y() -> Vec<f64> {
    let mut y = std::vec::Vec::new();
    if let Ok(lines) = read_lines("data/y.csv") {
        for line in lines {
            match line {
                Ok(ip) => {
                    if ip.contains("Iris-setosa") {
                        y.push(-1.);
                    } else if ip.contains("Iris-versicolor") {
                        y.push(1.);
                    } else{
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

fn main() {
    let pwd = std::env::current_dir().unwrap();
    let pwd_str = pwd.to_str().unwrap();
    print!("{}", pwd_str);
    let mut perceptron = Perceptron::new(0., 0., -1., 0., 1., 4);
    let x = get_iris_x();
    let y = get_iris_y();
    perceptron.train(x.clone(),y.clone(),100, 1.);


    println!("Hello, world!");
}
