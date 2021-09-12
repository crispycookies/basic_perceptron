use crate::node::{Node, unchecked_ops};

// This is an adapted Version of the Perceptron from:
// https://riptutorial.com/machine-learning/example/22618/implementing-a-perceptron-model-in-cplusplus
// Accessed 20.02.2021, 17:00-21:11

pub struct Perceptron<T> {
    m_nodes: std::vec::Vec<Node<T>>,
    m_zero: T,
    m_upper : T,
    m_lower : T,
    m_bias : T
}

impl<T: Copy + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + std::cmp::PartialOrd + unchecked_ops::UncheckedOps> Perceptron<T> {
    #[allow(dead_code)]
    pub fn new(def_weight: T, upper: T, lower : T, zero: T, size: usize) -> Self {
        let s = Perceptron {
            m_nodes: vec![Node::new(def_weight); size],
            m_zero: zero,
            m_upper: upper,
            m_lower: lower,
            m_bias : zero
        };
        return s;
    }
    #[allow(dead_code)]
    pub fn new_with_history(def_weight: T, upper: T, lower : T, zero: T, size: usize) -> Self {
        let s = Perceptron {
            m_nodes: vec![Node::new(def_weight); size+1],
            m_zero: zero,
            m_upper: upper,
            m_lower: lower,
            m_bias : zero
        };
        return s;
    }
    pub fn predict(&mut self, values: std::vec::Vec<T>) -> T{
        let mut numeric_prediction = self.m_bias;


        for i in 0..self.m_nodes.len(){
            let node = self.m_nodes.get(i).unwrap();
            let numeric_add = unchecked_ops::UncheckedOps::add(numeric_prediction, node.predict(values.get(i).unwrap().clone()));
            //numeric_prediction = numeric_prediction + node.predict(values.get(i).unwrap().clone());
            numeric_prediction = numeric_add;
        }

        if numeric_prediction > self.m_zero {
            return self.m_upper;
        }
        return self.m_lower;
    }
    pub fn train(&mut self, x: std::vec::Vec<std::vec::Vec<T>>, y: std::vec::Vec<T>, epochs: u64, eta: T) {
        if x.get(0).unwrap().len() != self.m_nodes.len() {
            panic!("Training Data X must match Node count");
        }
        for _ in 0..epochs {
            for j in 0..x.len(){


                let f_prediction= self.predict(x.get(j).unwrap().clone());

                let update_inner = unchecked_ops::UncheckedOps::sub(y.get(j).unwrap().clone(), f_prediction);
                let update = unchecked_ops::UncheckedOps::mul(eta, update_inner);

                for w in 0..self.m_nodes.len(){
                    let former = self.m_nodes.get_mut(w).unwrap().get_factor();
                    let inner_mul = unchecked_ops::UncheckedOps::mul(update, *x.get(j).unwrap().get(w).unwrap());
                    let outer_add = unchecked_ops::UncheckedOps::add(former, inner_mul);
                    self.m_nodes.get_mut(w).unwrap().set_factor(outer_add);
                }

                self.m_bias = update;
            }
        }
    }
}