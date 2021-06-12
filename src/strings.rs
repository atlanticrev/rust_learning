pub fn test() {
    // Strings are UTF-8 encoded!
    let mut s1 = "Some string literal".to_string(); // Due to string literals impls Display tarit, it can be converted to String
    s1.push_str("asd"); // Concatenate with string slice
    s1.push('!'); // Concatenate with one character
    println!("s1 string is: {}", s1);

    let mut s2 = String::from("some test string"); // Equals to "literal.to_string()"
    println!("s2 string is: {}", s2);

    let s2_new_slice = "asd";
    s2.push_str(s2_new_slice); // Concatenate with string slice
    println!("Updated s2 string is: {}", s2);
    println!("String slice that was added (s2_new_slice) is: {}", s2_new_slice);

    let s3 = String::from("some ");
    let s4 = String::from("value");
    let s3_s4_conc = format!("{}-{}", s3, s4); // format! does not take ownership
    println!("{}", s3_s4_conc);
    println!("{}", s3); // format! does not take ownership
    println!("{}", s4); // format! does not take ownership

    // 1. s3 moved to s5 (.add method takes ownership)
    // 2. s4 borrowed (because using .add method and it dictates that second param need to be &str)
    // 3. &String coerced to &str in &s4 by compiler (&s4 -> &4[..])
    // 4. "+" operator takes ownership of s3, adds copy of &s4 value to s3 string and returs ownership to s5
    let s5 = s3 + &s4;
    println!("{:?}, {:?}", s4, s5);
    for c in s5.chars() {
        println!("{}", c);
    }

    // String characters can not be accessed by int index!
    // Because UTF-8 is multibyte system, and accessing at single byte is not corrent here
}