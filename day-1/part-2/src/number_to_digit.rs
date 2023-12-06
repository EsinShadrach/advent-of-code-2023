use std::collections::HashMap;

pub fn get_digit(word: &str) -> Option<i32> {
    let mut number_map = HashMap::new();

    number_map.insert("one", 1);
    number_map.insert("two", 2);
    number_map.insert("three", 3);
    number_map.insert("four", 4);
    number_map.insert("five", 5);
    number_map.insert("six", 6);
    number_map.insert("seven", 7);
    number_map.insert("eight", 8);
    number_map.insert("nine", 9);

    match number_map.get(word) {
        Some(&value) => Some(value),
        None => None,
    }
}
