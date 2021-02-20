mod node;
mod perceptron;

fn main() {
    let mut node = node::Node::new(0.1);
    let _ = node.set_factor(0.2);
    let _p = node.predict(10.);

    println!("Hello, world!");
}
