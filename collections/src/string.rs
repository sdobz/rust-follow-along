pub fn demo() {
    let data = "String can be constructed with &str.to_string";
    let to_string = data.to_string();
    let on_literal = "or called on literal".to_string();
    let associated = String::from("or from associated ::from function");

    println!("{} {} {}", to_string, on_literal, associated);

    let hello = String::from("السلام عليكم"); // rtl issues?
    println!("{} world!", hello);

    let mut s = String::new();

    s.push_str("growt");
    s.push('h');

    let string_instance = String::from(" can happen");
    let s2 = s + &string_instance;
    // note: s has been moved to s2

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let most_dangerous_game = format!("{}-{}-{}", tic, tac, toe);

    println!("{} in {}", s2, most_dangerous_game);

    let cyrillic = String::from("Здравствуйте");
    let len = cyrillic.len();
    let chars = cyrillic.chars().count();

    println!(
        "You'd expect {}.len() to be {} long but it's actually {} (unicode scalars, bytes)",
        cyrillic, chars, len
    );
}

pub fn swap_case(subject: &str) -> String {
    subject
        .chars()
        .filter_map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().next()
            } else {
                c.to_lowercase().next()
            }
        })
        .collect()
}
