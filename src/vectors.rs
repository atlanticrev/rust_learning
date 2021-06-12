#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn test() {
    let mut v = Vec::new();
    let _v1 = vec![1, 2, 3];
    v.push(1);
    v.push(2);
    v.push(3);
    println!("vec[0] is: {}", &v[0]); // &v[i] - Gives a reference, if no element, program will crash

    match v.get(2) { // v.get(i) - Gives an Option<&T>, if no element, returns None, not crash
        Some(value) => println!("The third element is {}", value),
        None => println!("There are no third element")
    }
    for el in &v { // Readonly actions
        println!("{}", el);
    }
    for el in &mut v { // Make a changes
        *el += 1;
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for el in &row { // Readonly actions
        println!("{:?}", el);
    }
}