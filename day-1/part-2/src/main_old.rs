use std::fs;

fn main() {
    let mut total = 0;
    let data = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let data_splitted = data.split("\n");

    data_splitted.for_each(|item| {
        let mut string_sum = String::new();
        let digits: Vec<char> = item.chars().filter(|&c| c.is_digit(10)).collect();

        match digits.first() {
            Some(first_digit) => {
                let first_digit_str = first_digit.to_string();
                string_sum.push_str(&first_digit_str);
                println!("First => {}", first_digit);
            }
            _ => (),
        }
        match digits.last() {
            Some(last_digit) => {
                let last_digit_str = last_digit.to_string();
                string_sum.push_str(&last_digit_str);

                println!("Last => {}", last_digit);
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
