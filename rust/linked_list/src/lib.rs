use std::{cell::Cell, rc::Rc};

type NodePtr<T> = Option<Rc<Cell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: NodePtr<T>,
    prev: NodePtr<T>,
}

struct LinkedList<T> {
    length: u32,
    head: NodePtr<T>,
    tail: NodePtr<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            next: None,
            prev: None,
        }
    }

    pub fn link(&mut self, n: Node<T>) {}

    fn is_tail(&self) -> bool {
        self.next.is_none()
    }
    fn is_head(&self) -> bool {
        self.prev.is_none()
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    fn append(&mut self, data: T) {}
}
