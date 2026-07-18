// x [ 10 ] -> y [ 10005 ] -> z [9]

use std::boxed::Box;
use std::fmt::Display;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn add_node(&mut self, node: Node<T>) {
        match &mut self.next {
            Some(next_node) => next_node.add_node(node),
            None => self.next = Some(Box::new(node)),
        }
    }

    fn traverse(&self)
    where
        T: Display,
    {
        print!("{} ", self.value);
        if let Some(next_node) = &self.next {
            next_node.traverse()
        }
    }
    
    fn add(&mut self, value: T) {
        match &mut self.next {
            Some(next_node) => next_node.add(value),
            None => self.next = {
                let node = Node {
                    value,
                    next: None
                };
                Some(Box::new(node))
            }
        }
    }
}

fn main() {
    let x = Box::new(String::from("Naimish"));
    println!("{x}");
    let mut list = Node {
        value: 15,
        next: None,
    };
    let node = Node {
        value: 16,
        next: None,
    };
    list.add_node(node);
    list.traverse();
    println!();
    list.add(199);
    list.traverse();
    list.next.as_mut().unwrap().value = 15;
    list.traverse();
}
