extern crate ch14b_add_one;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        ch14b_add_one::add_one(num)
    );
}
