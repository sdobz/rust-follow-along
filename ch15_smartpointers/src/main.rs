mod refcell;
mod refcount;
mod weakrc;

use std::fmt;
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use self::List::{Cons, Nil};

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        self.fmt_recurse(f)
    }
}

impl List {
    fn fmt_recurse(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cons(v, next) => {
                write!(f, "{}", v)?;
                if let Nil = **next {
                    write!(f, "]")
                } else {
                    write!(f, ", ")?;
                    next.fmt_recurse(f)
                }
            }
            Nil => Ok(()),
        }
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List: {}", list);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // deref coercion
    hello(&(*m)[..]); // without

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("Custom Smart pointers created");

    refcount::demo();
    refcell::demo();
    weakrc::demo();
}

fn hello(name: &str) {
    println!("Hello {}!", name);
}
