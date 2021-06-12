// Lifecycle annotations is realationship between lifetimes of references
// Need to specify lifetime parameters for functions or structs that use references
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// It works without lifecycle annotations (explicit form - fn first_word<'a>(s: &'a str) -> &'a str {..})
// The patterns programmed into Rust’s analysis (compiler) of references are called the lifetime elision rules
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// There are three rules of lifecycle auto intefence

// 1. Each parameter that is a reference gets its own lifetime parameter (all lifecycles is different)
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
//    the lifetime of self is assigned to all output lifetime parameters.

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    // There are two input lifetimes, so Rust applies the first lifetime elision rule
    // and gives both &self and announcement their own lifetimes. Then, because one of the parameters is &self,
    // the return type gets the lifetime of &self, and all lifetimes have been accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn test() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    println!("{}", first_word("hello world"));

    // 'static lifetime means that this reference can live for the entire duration of the program
    let _s: &'static str = "I have a static lifetime.";
}