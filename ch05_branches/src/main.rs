fn main() {
    let number = 15.0001;

    if number % 5f64 == 0f64 {
        println!("condition was 5abl");
    } else if number % 3f64 == 0f64 {
        println!("condition was 3abl");
    } else {
        println!("{} is 5'nt & 3'nt", number);
    }

    let five = if true { 5 } else { 3 };

    let mut spooky_six = loop {
        println!("five is {}", five);
        break 8;
    };

    println!("spooky six is {}", spooky_six);

    println!("hold on fixing spooky six");

    while spooky_six > 6 {
        spooky_six -= 1;
        println!("thwack! {}", spooky_six);
    }

    let ordinary_vars = ['a', 'b', 'c', 'd', 'e'];

    println!("say hello to {} vars spooky!", ordinary_vars.len());

    for c in ordinary_vars.iter() {
        println!("{}: {}", spooky_six, c);
    }

    for n in (1..spooky_six).rev() {
        println!("{} is less than {}", n, spooky_six);
    }
}
