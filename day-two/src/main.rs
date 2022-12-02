use std::path::Path;
use std::fs::File;
use std::{env, io};
use std::io::BufRead;

fn main() {
    println!("Advent of code day 2");

    let current_dir = env::current_dir().expect("Attempt to get current dir");
    println!("current dir {:?}", current_dir);

    let file_path = current_dir.join(Path::new("input.txt"));
    println!("file path: {:?}", file_path);

    let mut total = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                // logic game
                let result: Vec<_> = line.split(" ").collect();
                let tup = (result[0], result[1]);
                let score_round = calculate_score(tup);
                println!("score of the round -> {}", score_round);
                total = total + score_round;
            }
        }
    }

    println!("Total score for tournament ==> {total}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
        _ => panic!("Wrong input")
    }
}