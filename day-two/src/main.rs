use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, io};

fn main() {
    println!("Advent of code day 2");

    let current_dir = env::current_dir().expect("Attempt to get current dir");
    let file_path = current_dir.join(Path::new("input.txt"));
    let result_puzzle_1 = puzzle_1(file_path.clone());
    let result_puzzle_2 = puzzle_2(file_path);

    println!("Result puzzle 1 ==> {result_puzzle_1}");
    println!("Result puzzle 2 ==> {result_puzzle_2}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn puzzle_1(file_path: PathBuf) -> i32 {
    let mut total = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                // logic game
                let result: Vec<_> = line.split(" ").collect();
                let tup = (result[0], result[1]);
                let score_round = calculate_score(tup);
                total = total + score_round;
            }
        }
    }

    total
}

fn puzzle_2(file_path: PathBuf) -> i32 {
    let mut total = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                // logic game
                let result: Vec<_> = line.split(" ").collect();
                let tup = (result[0], result[1]);
                let score_round = calculate_score_2(tup);
                total = total + score_round;
            }
        }
    }

    total
}

fn calculate_score(input_round: (&str, &str)) -> i32 {
    // A | X -> Rock :: score when selected 1
    // B | Y -> Paper :: score when selected 2
    // C | Z -> Scissors :: score when selected 3
    // lost -> 0
    // draw -> 3
    // won -> 6
    match input_round {
        ("A", "X") => 1 + 3,
        ("A", "Y") => 2 + 6,
        ("A", "Z") => 3 + 0,
        ("B", "X") => 1 + 0,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,
        ("C", "X") => 1 + 6,
        ("C", "Y") => 2 + 0,
        ("C", "Z") => 3 + 3,
        _ => panic!("Wrong input"),
    }
}

fn calculate_score_2(input_round: (&str, &str)) -> i32 {
    // A -> Rock :: score when selected 1
    // B -> Paper :: score when selected 2
    // C -> Scissors :: score when selected 3
    // X -> lost :: score 0
    // Y -> draw :: score 3
    // Z -> won :: score 6
    match input_round {
        ("A", "X") => 3 + 0, // adversary play rock, we have to loose (score 0) so we play scissors (score 3)
        ("A", "Y") => 1 + 3, // adversary play rock, we have to draw (score 3) so we play rock (score 1)
        ("A", "Z") => 2 + 6, // adversary play rock, we have to win (score 6) so we play paper (score 2)
        ("B", "X") => 1 + 0, // adversary play paper, we have to loose (score 0) so we play rock (score 1)
        ("B", "Y") => 2 + 3, // adversary play paper, we have to draw (score 3) so we play paper (score 2)
        ("B", "Z") => 3 + 6, // adversary play paper, we have to win (score 6) so we play scissors (score 3)
        ("C", "X") => 2 + 0, // adversary play scissors, we have to loose (score 0) so we play paper (score 2)
        ("C", "Y") => 3 + 3, // adversary play scissors, we have to draw (score 3) so we play scissors (score 3)
        ("C", "Z") => 1 + 6, // adversary play scissors, we have to win (score 6) so we play rock (score 1)
        _ => panic!("Wrong input"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_input_1() {
        let current_dir = env::current_dir().expect("Attempt to get current dir");
        let file_path = current_dir.join(Path::new("input.txt"));
        let result = puzzle_1(file_path);
        assert_eq!(result, 15523);
    }

    #[test]
    fn it_works_with_input_2() {
        let current_dir = env::current_dir().expect("Attempt to get current dir");
        let file_path = current_dir.join(Path::new("input.txt"));
        let result = puzzle_2(file_path);
        assert_eq!(result, 15702);
    }
}
