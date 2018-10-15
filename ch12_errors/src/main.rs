use std::fs;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem {:?}", e),
            },
            other_error => panic!("There was err {:?}", other_error),
        },
    };

    let _f = File::open("hello.txt").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem {:?}", error);
            })
        } else {
            panic!("There was err {:?}", error);
        }
    });

    let username = shortest_read_username().unwrap_or_else(|error| {
        panic!("Error reading username: {:?}", error);
    });

    println!("Username via hello.txt: {}", username);

    // let v = ValueInRange::new(101); // panics
    let v = ValueInRange::new(6);

    println!("{} is guaranteed to be in range 1-100", v.value());
}

fn _read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn _short_read_username() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn _shorter_read_username() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn shortest_read_username() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

struct ValueInRange {
    value: u32,
}

impl ValueInRange {
    pub fn new(value: u32) -> ValueInRange {
        if value < 1 || value > 100 {
            panic!("Value in range must be in range 1-100, got: {}", value);
        }

        ValueInRange { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
