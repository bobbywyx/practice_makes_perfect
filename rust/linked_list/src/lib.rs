use std::{rc::Rc, sync::Arc, ptr::NonNull};

struct LinkedList<T>{
    length: u32,
    head : Option<Box<Node<T>>>,
    tail : Option<Box<Node<T>>>,
}

struct Node<T>{
    data : T,
    next : Option<Box<Node<T>>>,
    prev : Option<Box<Node<T>>>
}

impl <T> Node<T> {
    fn new(data: T) -> Self {
        Node{
            data : data,
            next : None,
            prev : None,
        }
    }

    pub fn link(&mut self,n : Node<T>) {
        self.next = Some(Box::new(n));
    }

    fn is_tail(&self) -> bool {
        self.next.is_none()
    }
    fn is_head(&self) -> bool {
        self.prev.is_none()
    }
}

impl <T> LinkedList<T> {
    fn new() -> Self {
        LinkedList{
            length :0,
            head: None,
            tail: None,
        }
    }

    fn append(&mut self,  data: T){
        let new_node = Node::new(data);
        if self.length == 0{
            self.head = Some(Box::new(new_node));
        }else {
            
        }        
    }
}
