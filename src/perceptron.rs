use crate::node::Node;

// This is an adapted Version of the Perceptron from:
// https://riptutorial.com/machine-learning/example/22618/implementing-a-perceptron-model-in-cplusplus
// Accessed 20.02.2021, 17:00-21:11

pub struct Perceptron<T> {
    m_nodes: std::vec::Vec<Node<T>>,
    m_history: std::vec::Vec<T>,
    m_zero: T,
    m_upper : T,
    m_lower : T
}

impl<T: Copy + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::cmp::PartialOrd> Perceptron<T> {
    #[allow(dead_code)]
    pub fn new(def_weight: T, def_history: T, upper: T, lower : T, zero: T, size: usize) -> Self {
        let s = Perceptron {
            m_nodes: vec![Node::new(def_weight); size+1],
            m_history: vec![def_history; size+1],
            m_zero: zero,
            m_upper: upper,
            m_lower: lower
        };
        return s;
    }
    #[allow(dead_code)]
    pub fn new_with_history(def_weight: T, def_history: std::vec::Vec<T>, upper: T, lower : T, zero: T, size: usize) -> Self {
        let s = Perceptron {
            m_nodes: vec![Node::new(def_weight); size+1],
            m_history: def_history,
            m_zero: zero,
            m_upper: upper,
            m_lower: lower
        };
        return s;
    }
    pub fn predict(&mut self, values: std::vec::Vec<T>) -> bool {
        let mut numeric_prediction = self.m_nodes.get(0).unwrap().get_factor();


        for i in 1..self.m_nodes.len(){
            let node = self.m_nodes.get(i).unwrap();
            numeric_prediction = numeric_prediction + node.predict(values.get(i-1).unwrap().clone());
        }

        if numeric_prediction > self.m_zero {
            return true;
        }
        return false;
    }
    pub fn train(&mut self, x: std::vec::Vec<std::vec::Vec<T>>, y: std::vec::Vec<T>, epochs: u64, eta: T) {
        if x.get(0).unwrap().len() != self.m_nodes.len() -1 {
            panic!("Training Data X must match Node count");
        }
        for _ in 0..epochs {
            for j in 0..x.len(){
                let update_chunk_1 = eta * (y.get(j).unwrap().clone());

                let f_prediction;
                if self.predict(x.get(j).unwrap().clone()) {
                    f_prediction = self.m_lower;
                }else{
                    f_prediction = self.m_upper;
                }
                let update = update_chunk_1 - f_prediction;

                for w in 1..self.m_nodes.len(){
                    self.m_nodes.get_mut(w).unwrap().set_factor(update * *x.get(j).unwrap().get(w-1).unwrap());
                }

                self.m_nodes.get_mut(0).unwrap().set_factor(update);
            }
        }
    }
}