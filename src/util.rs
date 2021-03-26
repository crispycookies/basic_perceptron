use std::fmt::Debug;
use std::path::Path;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::collections::HashMap;

extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;
use std::iter::FromIterator;


pub struct Util<T> {
    size: usize,
    pub mapped_data: Vec<Vec<T>>,
    pub labeled_data: Vec<(Vec<T>, String)>,
    f_name: String,
    pub mapped: HashMap<String, T>,
    offset: T,
    min_map: T,
}

impl<T: std::str::FromStr> Util<T>
    where T: Copy + std::ops::Add<Output=T>
{
    pub fn new(size: usize, f_name: String, offset: T, min_map: T) -> Self
    {
        let s = Util {
            size,
            mapped_data: Vec::new(),
            labeled_data: Vec::new(),
            f_name,
            mapped: HashMap::new(),
            min_map,
            offset,
        };
        return s;
    }

    // https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
    // Accessed 20.02.2021, 21:30-22:31
    fn read_lines<P>(&self, filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    pub fn read_file(&mut self)
        where T: std::str::FromStr, <T as std::str::FromStr>::Err: Debug
    {
        if let Ok(lines) = self.read_lines(self.f_name.as_str()) {
            for line in lines {
                match line {
                    Ok(ip) => {
                        let tokens: Vec<&str> = ip.split(", ").collect();
                        if tokens.len() != self.size + 1 {
                            panic!("Invalid Data Format");
                        }
                        let mut v = std::vec::Vec::new();

                        for i in 0..self.size {
                            v.push(tokens.get(i).unwrap().to_string().parse::<T>().unwrap());
                        }

                        self.labeled_data.push((v, tokens.get(self.size).expect("No Label Provided").to_string()));
                    }
                    _ => {
                        panic!("Line could not be read");
                    }
                }
            }
        } else {
            panic!("File could not be found or opened");
        };
    }
    #[allow(dead_code)]
    pub fn generate_map(&mut self) {
        let mut it = self.min_map;
        for i in &self.labeled_data {
            if !self.mapped.contains_key(&*i.1) {
                self.mapped.insert(i.1.clone(), it);
                it = it + self.offset;
            }
        }
    }
    #[allow(dead_code)]
    pub fn map_file(&mut self) {
        for i in &self.labeled_data {
            let mut v = i.0.clone();
            v.push(*self.mapped.get(&*i.1.clone()).unwrap());
            self.mapped_data.push(v);
        }
    }
    #[allow(dead_code)]
    pub fn shuffle(&mut self) {
        self.mapped_data.shuffle(&mut thread_rng());
    }
    #[allow(dead_code)]
    pub fn cut(&self, len: usize) -> (Vec<Vec<T>>, Vec<Vec<T>>) {
        let training  = Vec::from_iter(self.mapped_data[0..len].iter().cloned());
        let validate = Vec::from_iter(self.mapped_data[len..self.mapped_data.len()].iter().cloned());

        return (training, validate);
    }
    #[allow(dead_code)]
    pub fn split_label_from_data(&self, data: Vec<Vec<T>>) -> (Vec<Vec<T>>, Vec<T>) {
        let mut raw_data = Vec::new();
        let mut label = Vec::new();
        for i in data {
            let vec_buff =Vec::from_iter(i[0..self.size].iter().cloned());
            raw_data.push(vec_buff);

            label.push(i.get(self.size).unwrap().clone());
        }
        return (raw_data, label);
    }
}