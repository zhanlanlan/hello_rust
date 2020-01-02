#![allow(dead_code)]

#[derive(Clone, Debug)]
struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
struct LinkedList<T> {
    len: usize,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            element: elem,
            next: None,
        }
    }

    fn set_next(&mut self, node: Self) {
        self.next = Some(Box::new(node));
    }

    fn get_last<'a>(&'a mut self) -> &'a mut Self {
        if let Some(ref mut x) = self.next {
            return x.get_last();
        }
        self
    }

    fn push(&mut self, elem: T) {
        let new_node = Node::new(elem);
        self.get_last().set_next(new_node);
    }
}

impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList {
            len: 0,
            next: Option::None,
        }
    }

    fn push(&mut self, element: T) {
        match &mut self.next {
            Option::None => self.next = Option::Some(Box::new(Node::new(element))),
            Option::Some(x) => x.push(element),
        };
        self.len += 1;
    }
}

pub fn run() {
    let mut l: LinkedList<isize> = LinkedList::new();
    l.push(123);
    l.push(43);
    l.push(432);
    l.push(4876);
    l.push(12);
    l.push(3213321);
    println!("{:?}", l);
}
