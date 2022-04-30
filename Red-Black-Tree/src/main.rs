struct Node<T: std::cmp::Ord>{
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    parent: Option<Box<Node<T>>>,
}

fn main() {
    println!("Hello, world!");
}
