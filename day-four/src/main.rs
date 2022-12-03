use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

fn main() {
    println!("Advent of code day 4");

    let _solved = parse_input_and_execute_puzzle_logic();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input_and_execute_puzzle_logic() -> i32 {
    let current_dir = env::current_dir().expect("Attempt to get current dir");
    println!("current dir {:?}", current_dir);

    let file_path = current_dir.join(Path::new("input.txt"));
    println!("file path: {:?}", file_path);

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                // logic puzzle
                let result = puzzle_logic(line);
            }
        }
    }
    0
}


fn puzzle_logic(input: String) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use crate::puzzle;

    #[test]
    fn it_works() {
        let result = puzzle();
        assert_eq!(result, None);
    }
}
