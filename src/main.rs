use core::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Head{
    head: Option<Rc<RefCell<Box<Node>>>>,
    tail: Option<Rc<RefCell<Box<Node>>>>,
    
}

#[derive(Debug)]
struct Node{
    content: u64,
    prev: Option<Rc<RefCell<Box<Node>>>>,
    next: Option<Rc<RefCell<Box<Node>>>>,
}

#[test]
fn main() {
    println!("Hello, world!");
    
    let node = Box::new(Node{content:1, prev:None, next:None});
    let node = Box::new(Node{content:2, prev:None, next:Some(Rc::new(RefCell::new(node)))});
    let node = Box::new(Node{content:3, prev:None, next:Some(Rc::new(RefCell::new(node)))});
    let node = Box::new(Node{content:4, prev:None, next:Some(Rc::new(RefCell::new(node)))});
    let node = Box::new(Node{content:5, prev:None, next:Some(Rc::new(RefCell::new(node)))});
    let head = Head{head:Some(Rc::new(RefCell::new(node))), tail:None};
    

    let mut dstr = Head::new();
    dstr.add(12);
    /*
    {
        let mut option = hola.next.borrow_mut();
        option.as_mut().unwrap().push('2');
    }
    */
    println!("{:#?}", &head);
    println!("{:#?}", &dstr);
    panic!();
}


impl Head{
    fn new() -> Self{
        Self{head:None, tail:None}
    }

    fn add(&mut self, content:u64) -> Result<(), ()> {
        println!("{:?}", self);
        match &self.head {
            Some(_) => {
                panic!();
                
            },
            None => {
                let holder = Box::new(Node{content:content, prev:None, next:None});
                let holder = Some(Rc::new(RefCell::new(holder)));
                self.head = holder;
                Ok(())
            }
        }
    }
    
}



