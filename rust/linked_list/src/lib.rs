use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

pub type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

// #[derive(Clone,Copy)]
pub struct Node<T> {
    data: T,
    next: NodePtr<T>,
    prev: NodePtr<T>,
}

pub struct LinkedList<T> {
    pub length: u32,
    pub head: NodePtr<T>,
    pub tail: NodePtr<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            next: None,
            prev: None,
        }
    }

    pub fn get_data(&self) -> &T {
        &self.data
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
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn append(&mut self, data: T) {
        let new_node_ptr = Rc::new(RefCell::new(Node::new(data)));

        match &mut self.tail {
            Some(ptr) => {
                ptr.borrow_mut().next = Some(new_node_ptr.clone());
            }
            None => {
                self.head = Some(new_node_ptr.clone());
            }
        }
        self.tail = Some(new_node_ptr.clone());
    }
}
