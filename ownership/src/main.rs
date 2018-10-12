fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world");
    
    // let s2 = s1;
    // println!("{}", s1); borked because move

    let s2 = s1.clone();
    println!("My copy: {}", s1);

    take_ownership(s2);
    // println!("{}", s2); borked because move

    let x = 5;
    make_copy(x);
    println!("Still have {}", x);

    let my_s = give_ownership();
    println!("Got: {}", my_s);

    // take_and_give(my_s); // ignores return
    // println!("Still have {}", my_s); // borked because gave away

    let same_s = take_and_give(my_s);
    println!("Still have: {}", same_s);

    let (same_s, len_s) = take_and_give_with_improvment(same_s);

    println!("The '{}' has {} lens", same_s, len_s);

    let unceremonious_len = borrow_and_improve(&same_s);

    println!("With less ado it has: {} lens", unceremonious_len);

    let mut mut_s = same_s;
    // println!("Do I still have: {}?", same_s); // nope

    cause_mutation_on(&mut mut_s);
    println!("Uh oh, now: {}", mut_s);

    let _mut_ref1 = &mut mut_s;
    //let mut_ref2 = &mut mut_s; // borked, cannot borrow as mutable more than 1 at a time
    //println!("{} and {}", _mut_ref1, mut_ref2);
}

fn take_ownership(s: String) {
    println!("I got yer \"{}\"", s);
}

fn make_copy(i: i32) {
    println!("Your {} eludes me", i);
}

fn give_ownership() -> String {
    String::from("here it is")
}

fn take_and_give(s: String) -> String {
    println!("Handling: {}", s);
    s
}

fn take_and_give_with_improvment(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn borrow_and_improve(s: &String) -> usize {
    s.len()
}

fn cause_mutation_on(s: &mut String) {
    s.push_str(", with superpowers");
}