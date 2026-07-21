use ::std::boxed::Box;
use ::std::fmt::Display;

// Lets Implement a linked list with Boxes
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn add_node(&mut self, node: Node<T>) {
        match &mut self.next {
            Some(next_node) => next_node.add_node(node),
            None => self.next = Some(Box::new(node)),
        };
    }

    fn add(&mut self, value: T) {
        match &mut self.next {
            Some(next_node) => next_node.add(value),
            None => {
                self.next = {
                    let node = Node { value, next: None };
                    Some(Box::new(node))
                }
            }
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
}

fn main() {
    let mut x = Box::new(String::new());
    x.push_str("Naimish");
    println!("{x}");
    let mut list = Node {
        value: 10,
        next: None,
    };
    let element = Node {
        value: 15,
        next: None,
    };
    list.add_node(element);
    list.add(16);
    list.traverse();
}
