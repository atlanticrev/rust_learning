use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

// Error propagation

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = match File::open("hello.txt") {
        Ok(fd) => fd,
        Err(err) => return Err(err),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(err),
    }
}

fn read_username_from_file_shorthand() -> Result<String, io::Error> {
    // 1. "?" operator converts error type into the error type defined in the return type of the current function
    // 2. Each error type must implement From trait to be converted in "?" operator
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorthand2() -> Result<String, io::Error> {
    let mut s = String::new();
    // "?" chaining shorthand
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_shorthand3() -> Result<String, io::Error> {
    // Super shorthand
    fs::read_to_string("hello.txt")
}

// The ? Operator Can Be Used in Functions That Return Result

pub fn test() {
    // let vec = vec![1, 2, 3];
    // vec[99]; // panic!

    // Matching on Different Errors
    // Using matches
    // let _f = match File::open("hello.txt") {
    //     Ok(result) => result,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fd) => fd,
    //             Err(err) => panic!("Problem creating the file: {:?}", err)
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error)
    //         }
    //     },
    // };

    // Using closures
    let _f1 = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect
    let _f2 = File::open("hello.txt").unwrap(); // Returns value if Ok() and panics if Err()
    let _f3 = File::open("hello.txt").expect("Failed to open hello.txt"); // Similar to .unwrap() and adds custom error message
}