extern crate binary_tree;

use binary_tree::Node;

fn main() {
    let mut root = Node::new();

    for i in 1..11 {
        root.insert(i);
    }

    println!("{:?}", root.search(&6));

//    println!("{:?}", root);
}
