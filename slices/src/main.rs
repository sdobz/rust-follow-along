fn main() {
    let word_soup = String::from("This is word soup");

    println!("The first word in '{}' is at index {}", word_soup, first_word_loc(&word_soup));

    let w = &word_soup[0..4]; // or [0..=3]
    println!("Retrieved manually: {}", w);

    let w = first_word(&word_soup);
    println!("Slice and diced: {}", w);
}

fn first_word_loc(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
    // s?
}