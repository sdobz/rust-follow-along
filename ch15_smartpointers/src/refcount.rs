use std::rc::Rc;

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}
use self::RcList::{Cons, Nil};

pub fn demo() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Ref count after a: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Ref count after b: {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Ref count after c: {}", Rc::strong_count(&a));
        println!("c: {:?}", c);
    }
    println!("Ref count after c out of scope: {}", Rc::strong_count(&a));

    println!("b: {:?}", b);
}
