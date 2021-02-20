#[derive(Copy, Clone)]
pub struct Node<T>{
     m_weight: T
}

impl<T: Copy + std::ops::Mul<Output = T>> Node<T> {
    pub fn set_factor(&mut self, weight:T){
        self.m_weight = weight;
    }
    pub fn predict(&self, value:T) -> T {
        return value * self.m_weight;
    }
    pub fn new(weight: T) -> Self{
        let s = Node {
            m_weight: weight
        };
        return s;
    }
    #[allow(dead_code)]
    pub fn get_factor(&self) -> T{
        return self.m_weight;
    }
}