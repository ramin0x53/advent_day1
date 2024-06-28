use regex::Regex;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn find_calibration_value_str_number(text: &str) -> i64 {
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let mut digit_strings: Vec<String> = re
        .find_iter(text)
        .map(|mat| mat.as_str().to_string())
        .collect();

    println!("{:?}", digit_strings);
    for elem in digit_strings.iter_mut() {
        match &elem[..] {
            "one" => *elem = String::from("1"),
            "two" => *elem = String::from("2"),
            "three" => *elem = String::from("3"),
            "four" => *elem = String::from("4"),
            "five" => *elem = String::from("5"),
            "six" => *elem = String::from("6"),
            "seven" => *elem = String::from("7"),
            "eight" => *elem = String::from("8"),
            "nine" => *elem = String::from("9"),
            _ => (),
        }
    }

    println!("{:?}", digit_strings);
    println!("================================");
    let mut result: String = "".to_string();
    if let (Some(first), Some(last)) = (digit_strings.first(), digit_strings.last()) {
        result = format!("{}{}", first, last);
    } else {
        println!("No digits found in the text");
    }

    let mut result_int: i64 = 0;
    match result.parse::<i64>() {
        Ok(result) => result_int = result,
        Err(e) => println!("Failed to convert to i64: {}", e),
    }
    result_int
}

fn _find_calibration_value(text: &str) -> i64 {
    let re = Regex::new(r"\d").unwrap();
    let digit_strings: Vec<String> = re
        .find_iter(text)
        .map(|mat| mat.as_str().to_string())
        .collect();

    let mut result: String = "".to_string();
    if let (Some(first), Some(last)) = (digit_strings.first(), digit_strings.last()) {
        result = format!("{}{}", first, last);
    } else {
        println!("No digits found in the text");
    }

    let mut result_int: i64 = 0;
    match result.parse::<i64>() {
        Ok(result) => result_int = result,
        Err(e) => println!("Failed to convert to i64: {}", e),
    }
    result_int
}

fn main() {
    let res = read_lines("./input.txt");
    let mut sum: i64 = 0;
    for txt in &res {
        sum += find_calibration_value_str_number(txt);
    }
    println!("Sum: {:?}", sum);
}
