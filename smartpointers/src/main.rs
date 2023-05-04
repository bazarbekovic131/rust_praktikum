use crate::List::{Cons,Nil};
use crate::SharedList::{Cons,Nil};
use std::rc::Rc; //use Reference counter smart pointer
use std::cell::RefCell;

fn main() {
    //Boxes allow to store data on the heap rather than on stack

    let b = Box::new(5);
    println!("b = {}", b);
    
    let list = Cons(1, Box::new(Cons(2,Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let p = &x;
    let sp = Box::new(x);

    let a = Rc::new(Cons(5, Rc::new(10, Rc::new(Nil))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("Number of refs to list a: {}", Rc::strong_count(&a));

    //tree data struktur:

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum SharedList {
    Cons(i32, Rc<SharedList>),
    Nil,
}

struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; //defines associated type for the deref trait

    fn deref(&self) -> &Self::Target {
        &self.0 //returns ref to the value with *
    }
}

//tree data struktur

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: Refcell<Vec<Rc<Node>>>,
}