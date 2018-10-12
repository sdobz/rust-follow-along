pub fn demo_all() {
    demo_init();
    demo_multitype_vector();
}

fn demo_init() {
    let v_generic_new: Vec<i32> = Vec::new();
    let v_turbo_fish = Vec::<i32>::new();
    let v_macro = vec![1, 2, 3];
    println!(
        "Constructions: {:?} {:?} {:?}",
        v_generic_new, v_turbo_fish, v_macro
    );

    let mut v = Vec::new();
    v.push(1); // holy inference batman!
    v.push(2);
    v.push(3);

    let third: &i32 = &v[2];
    let also_third = v[2];
    println!(
        "Third: {} also third: {} owner?: {:?}",
        third, also_third, v
    );

    let mut v_index = 0;
    loop {
        match v.get(v_index) {
            Some(contents) => {
                println!("[{}] Found value: {}", v_index, contents);
            }
            None => {
                println!("Unreachable");
                break;
            }
        }
        v_index += 1;
    }

    if let Some(_) = v.get(1000) {
        println!("Unreachable code");
    }
    // let unreachable_code = &v[100]; // causes panic

    let mut v2 = vec![1,2,3];
    let _first = &v2[0];

    v2.push(4); // book says this shouldnt work, push borrows mutably and _first is immutable
    println!("The heck? {:?}", v);

    for i in &v2 {
        println!("Found value {}", i);
    }

    for i in &mut v2 {
        *i += 50;
    }
    println!("Embiggened: {:?}", v2);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn demo_multitype_vector() {
    use self::SpreadsheetCell::*;

    let row = vec![
        Int(3),
        Text(String::from("blu")),
        Float(10.12),
    ];

    for cell in &row {
        match cell {
            Int(v) => println!("Int Cell: {}", v),
            Text(v) => println!("Text Cell: {}", v),
            Float(v) => println!("Float Cell: {}", v),
        }
    }
}