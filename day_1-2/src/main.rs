use std::env;
use std::fs;
use std::fmt;
use regex::Regex;

#[derive(Debug, Clone)]
struct StringDigitError {
    digit: String,
}

impl fmt::Display for StringDigitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid digit string {} was passed to conv_digit", self.digit)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    println!("with text:\n{}", contents);

    let result = solution(&contents);

    println!("result: {}", result);
}

fn solution(content: &String) -> i32 { 
    let lines = content.lines();

    lines.map(|line| collect_digits(&line.to_string()))
        .fold(0, |acc, digits| {acc + digits}) 
}

fn conv_digit(digit: &str) -> Result<i32, StringDigitError> {
    let parsed = digit.parse::<i32>();

    match parsed {
        Ok(value) => Ok(value),
        Err(_) =>
            match digit {
                "one"   | "eno"   => Ok(1),
                "two"   | "owt"   => Ok(2),
                "three" | "eerht" => Ok(3),
                "four"  | "ruof"  => Ok(4),
                "five"  | "evif"  => Ok(5),
                "six"   | "xis"   => Ok(6),
                "seven" | "neves" => Ok(7),
                "eight" | "thgie" => Ok(8),
                "nine"  | "enin"  => Ok(9),
                _ => Err(
                    StringDigitError { 
                        digit: digit.to_string() 
                    }
                ),
            }
    }
}

fn collect_digits(line: &String) -> i32 {
    let line_reversed = line.chars().rev().collect::<String>();

    let re_forward = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    
    let re_backward = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    let match_forward = re_forward.find(line.as_str()).unwrap().as_str();

    let match_backward = re_backward.find(line_reversed.as_str()).unwrap().as_str();

    let first = conv_digit(match_forward).unwrap();

    let last = conv_digit(match_backward).unwrap();

    let number = format!("{}{}", first, last);

    number.parse::<i32>().unwrap()
}
