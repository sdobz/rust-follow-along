const MAX_POINTS: u128 = 200_000;

fn main() {
    let x = 5;
    println!("Value is: {}", x);

    let tup = (3, 6.5, 8u128);
    let (x, _, _) = tup;

    println!("Value is: {}", x);

    let a = [1, 2, 3];

    let element = a[x];

    println!("Value of element: {}", element);
}
