use std::collections::HashMap;

pub fn test() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 1);
    scores.insert(String::from("Red"), 0);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // 1. .into_inter - Get iterator of a vector
    // 2. .zip - Creates vector of tuples of key-value format
    // 3. .collect - Vector to HashMap
    // 4. "HashMap<_, _>" - Specify that we need to collect to HashMap
    let mut scores_with_collect: HashMap<_, _> 
        = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // If we insert references to values into the hash map, the values won’t be moved into the hash map. 
    // The values that the references point to must be valid for at least as long as the hash map is valid.

    match scores_with_collect.get("Blue") {
        Some(&value) => println!("{}", value),
        _ => println!("Invalid value")
    }

    for (key, value) in &scores_with_collect {
        println!("{}: {}", key, value);
    }

    // Overwriting a Value
    scores_with_collect.insert(String::from("Blue"), 123); // Inserting twice same entry replacing it
    for (key, value) in &scores_with_collect {
        println!("{}: {}", key, value);
    }

    // Only Inserting a Value If the Key Has No Value
    scores_with_collect.entry(String::from("Green")).or_insert(456);
    for (key, value) in &scores_with_collect {
        println!("{}: {}", key, value);
    }

    // Updating a Value Based on the Old Value
    let value = scores_with_collect.entry(String::from("Blue")).or_insert(456);
    *value += 1; // Write to mut reference, using previous value
    for (key, value) in &scores_with_collect {
        println!("{}: {}", key, value);
    }
}