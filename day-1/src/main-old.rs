use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Failed to read Input.txt");
    let split_data = data.split("\n");
    let mut total: u32 = 0;

    split_data.for_each(|i| {
        println!("{}", i);
        println!("");
        let mut two_digits = 0;
        let digits: Vec<char> = i.chars().filter(|&c| c.is_digit(10)).collect();
        if let Some(first_digit) = digits.first() {
            if let Some(first_digit_val) = first_digit.to_digit(10) {
                two_digits += first_digit_val;
                println!("First Digit => {}", first_digit_val);
            }
        }
        if let Some(last_digit) = digits.last() {
            if let Some(last_digit_val) = last_digit.to_digit(10) {
                two_digits += last_digit_val;
                println!("Last Digit => {}", last_digit_val);
            }
        }
        println!("Two Digits => {}", two_digits);
        total += two_digits;
    });
    println!("{}", total);
}
