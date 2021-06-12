fn takes_ownership(some_string: String) {
    println!("{}", some_string); // "some_string" will be freed at the end of this scope (because of moving)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello two"); // "some_string" will be moved to the caller scope
    some_string
}

fn get_len(s: &String) -> usize {
    s.len() // Refers to the value of "str" but does not owns it (borrowing)
}

fn print_chars(s: &str) {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        println!("element [{}] is {}", i, item);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

pub fn test() {
    let s = String::from("hello"); // String is heap allocated data
    println!("{}", s);

    let s1 = s; // "s" moved to "s1" (Moving works only with heap allocated data), "s" is no longer valid!
    println!("{}", s1);

    let s2 = s1.clone(); // "s1" Copied with heap data to "s2"
    println!("{}", s2);

    takes_ownership(s1); // "s1" moved to "takes_ownership"s argument - "some_string"

    let n = 5; // Copy of primitive types (no moving)
    let n1 = n; // "n" not moving here, but copies
    println!("{}, {}", n, n1);

    let str = gives_ownership(); // Takes ownnership form a function
    println!("{}", str);
    let str_len = get_len(&str); // Borrowing of "str" value, not moving it, refs do not ownes values
    println!("Length of the string is: {}", str_len);
    print_chars(&str); // Function "print_chars" borrowing the value

    let str1 = String::from("test");
    let str1_slice = &str1[0..3];
    println!("{:?}, {:?}, {:?}", first_word(&str1), first_word(&str1_slice), first_word("super word")); // String literals is a &str actually
}
