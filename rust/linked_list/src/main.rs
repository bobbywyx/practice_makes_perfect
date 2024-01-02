use std::{rc::Rc, cell::RefCell};

use linked_list;
fn main() {
    println!("Hello, world!");

    let mut list = linked_list::LinkedList::new();
    list.append(2);
    list.append(3);
    list.append(4);

    println!("{}",list.head.unwrap().borrow().get_data());
    println!("{}",list.tail.unwrap().borrow().get_data());
}
