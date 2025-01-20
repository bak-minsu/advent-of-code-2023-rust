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

    lines.map(|line| game_sum(&line.to_string())) 
        .fold(0, 
            |acc, result| {
                match result {
                    None => acc,
                    Some(id) => acc + id,
                }
            }
        )
}

fn game_sum(line: &String) -> Option<i32> {
    let results = game_results(line);

    if results.iter().any(|game| !game.valid()) {
        return None;
    }

    Some(game_number(line))
}

#[derive(Clone)]
enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Clone)]
struct CubeResult {
    count: i32,
    color: CubeColor,
}

impl CubeResult {
   fn valid(&self) -> bool {
       match self.color {
           CubeColor::Red => self.count <= 12,
           CubeColor::Green => self.count <= 13,
           CubeColor::Blue => self.count <= 14,
       }
   }
}

struct CubeGame {
    results: Vec<CubeResult>,
}

impl CubeGame {
    fn valid(&self) -> bool {
        self.results.iter().all(|result| result.valid())
    }
}

fn game_results(line: &String) -> Vec<CubeGame> {
    let results_exp = Regex::new(r" (?<count>\d+) (?<color>red|green|blue)(?<separator>[,;]?)").unwrap();

    let mut games: Vec<CubeGame> = vec![];
    let mut current_results: Vec<CubeResult> = vec![];

    let captures = results_exp.captures_iter(line);

    captures.for_each(
        |cap| {
            let count = cap.name("count").unwrap().as_str().parse::<i32>().unwrap();

            let color = cap.name("color").unwrap().as_str();

            let cube_color = match color {
                "red" => CubeColor::Red,
                "green" => CubeColor::Green,
                "blue" => CubeColor::Blue,
                _ => panic!("Invalid color"),
            };

            current_results.push(
                CubeResult { 
                    count: count, 
                    color: cube_color,
                }
            );

            if cap.name("separator").unwrap().as_str() == ";" || cap.name("separator").unwrap().as_str() == "" {
                games.push(
                    CubeGame { results: current_results.clone() }
                );

                current_results.clear(); 
            }

       }
    );

    games
}

fn game_number(line: &String) -> i32 {
    let game = Regex::new(r"Game (?<id>\d+):").unwrap();

    let id = game.captures(line).unwrap().name("id").unwrap().as_str();

    id.parse::<i32>().unwrap()
}
