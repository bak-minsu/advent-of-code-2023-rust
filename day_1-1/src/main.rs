use std::env;
use std::fs;
use regex::Regex;

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

fn collect_digits(content: &String) -> i32 {
    let re = Regex::new(r"\d").unwrap();

    let digits: Vec<&str> = re.find_iter(content).map(|m| m.as_str()).collect();

    let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());

    number.parse::<i32>().unwrap()
}
