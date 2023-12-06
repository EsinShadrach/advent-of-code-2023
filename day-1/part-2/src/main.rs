// use std::fs;

// pub mod main_old;
mod number_to_digit;

fn main() {
    let mut total = 0;
    let number_word = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let _total: i32 = 0;
    let sample_data = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];
    // let data = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    // let data_splitted = data.split("\n");
    let sample_data_iter = sample_data.iter().enumerate();
    sample_data_iter.for_each(|(_, item)| {
        println!("{}", item);
        let mut string_sum = String::new();

        let mut number_word_indices: Vec<(usize, &str)> = number_word
            .iter()
            .filter_map(|&num| {
                // Item.find returns a
                match item.find(num) {
                    Some(index) => Some((index, num)),
                    None => None,
                }
            })
            .collect();

        number_word_indices.sort_by_key(|&(index, _)| index);

        match number_word_indices.first() {
            Some((_, first_word)) => {
                let return_digit = return_digit(&first_word);
                match return_digit {
                    Ok(digit) => {
                        println!("first => {}", digit);
                        string_sum.push_str(&digit.to_string());
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
            }
            _ => (),
        }
        match number_word_indices.last() {
            Some((_, last_word)) => {
                let return_digit = return_digit(&last_word);
                match return_digit {
                    Ok(digit) => {
                        println!("Last => {}", digit);
                        string_sum.push_str(&digit.to_string());
                    }
                    Err(err) => {
                        eprintln!("Error: {}", err);
                    }
                }
            }
            _ => (),
        }
        if string_sum.is_empty() == false {
            let str_sum_to_num = string_sum
                .parse::<i32>()
                .expect("Failed to convert `string_sum` to num");
            total += str_sum_to_num;
        }
        println!()
    });
    println!("{}", total)
}

fn return_digit(word: &str) -> Result<i32, std::io::Error> {
    let num_to_digit = number_to_digit::get_digit(word);
    match num_to_digit {
        Some(digit) => {
            return Ok(digit);
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Digit Not Found",
            ));
        }
    }
}
