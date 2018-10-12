fn main() {
    println!("Hello, world!");

    another_function(5);
}

fn another_function(x: i32) {
    println!("Another function: {}", x);

    let y = {
        let z = 2;
        z + add_one_to(five())
    };

    println!("{}", y);
}

fn five() -> i32 {
    5
}

fn add_one_to(x: i32) -> i32 {
    x + 1
}