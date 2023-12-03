use std::fs;
mod number_to_digit;

fn main() {
    let number_word = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut total = 0;
    let sample_data = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];
    let data = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let _data_splitted = data.split("\n");
    let sample_data_iter = sample_data.iter().enumerate();

    sample_data_iter.for_each(|(_index, item)| {
        println!("{}", item);
        let mut string_sum = String::new();
        let contains_num_word = number_word
            .iter()
            .find(|&&num_word| item.contains(num_word));

        match contains_num_word {
            Some(word) => {
                println!("Found word: {}", word);
                match number_to_digit::get_digit(word) {
                    Some(digit) => println!("{}", digit),
                    None => (),
                }
            }
            None => println!("Excuting None"),
        }
        let digits: Vec<char> = item.chars().filter(|&c| c.is_digit(10)).collect();
        match digits.first() {
            Some(first_digit) => {
                let first_digit_str = first_digit.to_string();
                string_sum.push_str(&first_digit_str);
                // println!("First => {}", first_digit);
            }
            _ => (),
        }
        match digits.last() {
            Some(last_digit) => {
                let last_digit_str = last_digit.to_string();
                string_sum.push_str(&last_digit_str);
                // println!("Last => {}", last_digit);
            }
            _ => (),
        }
        if string_sum.is_empty() == false {
            let string_sum_to_num = string_sum
                .parse::<u32>()
                .expect("Failed to parse variable string_sum as a number");
            total += string_sum_to_num;
        }
    });
    println!("{}", total);
}
