use std::fmt::Debug;
use std::path::Path;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::iter::Map;
use std::collections::BTreeMap;

pub struct Util<T> {
    size: usize,
    mapped_data: Vec<Vec<T>>,
    labeled_data: Vec<(Vec<T>, String)>,
    f_name: String,
    mapped: BTreeMap<String, T>,
    offset : T,
    min_map : T
}

impl<T: std::str::FromStr> Util<T>
    where T: Ord + Copy + std::ops::Add<Output = T>
{
    pub fn new(size: usize, f_name: String, offset : T, min_map : T) -> Self
    {
        let s = Util {
            size,
            mapped_data: Vec::new(),
            labeled_data: Vec::new(),
            f_name,
            mapped: BTreeMap::new(),
            offset,
            min_map
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
    pub fn generate_map(&mut self) {
        let mut it = self.min_map;
        for i in &self.labeled_data {
            if !self.mapped.contains_key(&*i.1) {
                self.mapped.insert(i.1.clone(), it);
                it = it + self.offset;
            }
        }
    }
    pub fn map_file(&mut self){
        for i in &self.labeled_data {

        }
    }
}