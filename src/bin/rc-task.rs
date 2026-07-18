use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn add_behind(this: &Rc<RefCell<Self>>, value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            next: Some(Rc::clone(this)),
        }))
    }

    fn traverse(this: &Rc<RefCell<Self>>)
    where
        T: Display,
    {
        let node = this.borrow();
        print!("{} ", node.value);
        if let Some(next_node) = &node.next {
            Node::traverse(next_node)
        }
    }

    fn add_front(this: &Rc<RefCell<Self>>, value: T) {
        let mut node = this.borrow_mut();
        match &node.next {
            Some(next) => Node::add_front(next, value),
            None => node.next = Some(Rc::new(RefCell::new(Node { value, next: None }))),
        }
    }
}

fn main() {
    let mut list_b = Rc::new(RefCell::new(Node {
        value: 4,
        next: None,
    }));
    let mut list_a = list_b.clone();
    for i in (1..4).rev() {
        list_a = Node::add_behind(&list_a, i);
    }
    Node::traverse(&list_a);
    println!();
    println!("{}", Rc::strong_count(&list_b));
    for i in (51..54).rev() {
        list_b = Node::add_behind(&list_b, i);
    }
    println!("{}", Rc::strong_count(&list_b));
    Node::traverse(&list_b);
    println!();
    Node::add_front(&list_a, 69);
    Node::traverse(&list_b);
    Node::add_front(&list_b, 420);
    println!();
    Node::traverse(&list_a);

}













