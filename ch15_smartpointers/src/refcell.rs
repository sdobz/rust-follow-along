pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max > 0.75 && percentage_of_max < 0.9 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota");
        } else if percentage_of_max >= 0.1 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum RcRefCellList {
    Cons(Rc<RefCell<i32>>, Rc<RcRefCellList>),
    Nil,
}

#[derive(Debug)]
enum TailList {
    Cons(i32, RefCell<Rc<TailList>>),
    Nil,
}

impl TailList {
    fn tail(&self) -> Option<&RefCell<Rc<TailList>>> {
        match self {
            TailList::Cons(_, item) => Some(item),
            TailList::Nil => None,
        }
    }
}

use self::RcRefCellList::{Cons, Nil};

pub fn demo() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    let a = Rc::new(TailList::Cons(5, RefCell::new(Rc::new(TailList::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(TailList::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Will stack overflow
    // println!("a next item = {:?}", a.tail());
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    #[should_panic]
    fn it_panics_when_multi_mut_borrow() {
        let ref_cell = RefCell::new(6);

        let mut _one_borrow = ref_cell.borrow_mut();
        let mut _two_borrow = ref_cell.borrow_mut();
    }
}
