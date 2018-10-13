use std::collections::HashMap;

use super::string::swap_case;

pub fn demo() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("bLUE"), String::from("yELLOW")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name: String = swap_case(&String::from("Yellow"));

    let score = scores.get(&team_name);
    if let Some(score) = score {
        println!("{} got {} points", team_name, score);
    } else {
        println!("Did not find {} in scores", team_name);
    }

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
        };
    );

    let hashmap_literal = map!{
        String::from("blu") => 10,
        String::from("red") => 20
    };

    for (team_name, points) in &hashmap_literal {
        println!("{} got {} points", team_name, points);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(666);

    println!("Using entry {:?}", scores); // no 666

    let text = "hello world wonderful world";
    let occurrences = count_word_occurrence(text);
    println!("{} found:\n{:?}", text, occurrences);
}

fn count_word_occurrence(s: &str) -> HashMap<&str, i32> {
    let mut map = HashMap::new();

    for word in s.split_whitespace() {
        let count = map.entry(word).or_insert(0i32);
        *count += 1;
    }

    map
}
