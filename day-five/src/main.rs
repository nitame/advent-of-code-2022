use std::fs::File;
use std::io::BufRead;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::{env, io};

fn main() {
    println!("Advent of code day 5");

    let crates = vec![
        String::from("NDMQBPZ"),
        String::from("CLZQMDHV"),
        String::from("QHRDVFZG"),
        String::from("HGDFN"),
        String::from("NFQ"),
        String::from("DQVZFBT"),
        String::from("QMTZDVSH"),
        String::from("MGFPNQ"),
        String::from("BWRM"),
    ];

    let file_path = get_file_path(String::from("input.txt"));
    let result = puzzle_2(file_path, crates);

    println!("result -> {result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_file_path(filename: String) -> PathBuf {
    let current_dir = env::current_dir().expect("Attempt to get current dir");
    current_dir.join(Path::new(filename.as_str()))
}

fn puzzle(file_path: PathBuf, crates: Vec<String>) -> String {
    let mut top_crates = String::new();
    let mut clone_crate = crates.clone();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                // logic puzzle
                let instructions: Vec<i32> = line
                    .replace("move ", "")
                    .replace(" from ", ",")
                    .replace(" to ", ",")
                    .split(",")
                    .map(|item| item.parse::<i32>().unwrap())
                    .collect();
                let mut number_of_crates_to_move = instructions[0];
                let move_crates_from_column = instructions[1] - 1;
                let move_crates_to_column = instructions[2] - 1;
                let mut pop_from = clone_crate
                    .get(move_crates_from_column as usize)
                    .unwrap()
                    .to_string();
                let mut push_to = clone_crate
                    .get(move_crates_to_column as usize)
                    .unwrap()
                    .to_string();
                while number_of_crates_to_move > 0 {
                    let crate_to_move = pop_from.pop().unwrap();
                    push_to.push(crate_to_move);
                    number_of_crates_to_move = number_of_crates_to_move - 1;
                }
                clone_crate[move_crates_from_column as usize] = pop_from.clone();
                clone_crate[move_crates_to_column as usize] = push_to.clone();
            }
        }
    }
    for pile in clone_crate.iter() {
        let top_pile = pile.to_string().pop().unwrap();
        top_crates.push(top_pile);
    }
    top_crates.to_string()
}

fn puzzle_2(file_path: PathBuf, crates: Vec<String>) -> String {
    let mut top_crates = String::new();
    let mut clone_crate = crates.clone();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                // logic puzzle
                let instructions: Vec<i32> = line
                    .replace("move ", "")
                    .replace(" from ", ",")
                    .replace(" to ", ",")
                    .split(",")
                    .map(|item| item.parse::<i32>().unwrap())
                    .collect();
                let number_of_crates_to_move = instructions[0] as usize;
                let move_crates_from_column = instructions[1] - 1;
                let move_crates_to_column = instructions[2] - 1;
                let mut take_crates_from = clone_crate
                    .get(move_crates_from_column as usize)
                    .unwrap()
                    .to_string();
                let push_crates_to = clone_crate
                    .get(move_crates_to_column as usize)
                    .unwrap()
                    .to_string();
                let pile_size = take_crates_from.len();
                let crates_to_move: String = take_crates_from
                    .drain(pile_size - number_of_crates_to_move..pile_size)
                    .collect();
                clone_crate[move_crates_from_column as usize] = take_crates_from.clone();
                clone_crate[move_crates_to_column as usize] =
                    push_crates_to.add(crates_to_move.as_str());
            }
        }
    }
    for pile in clone_crate.iter() {
        let top_pile = pile.to_string().pop().unwrap();
        top_crates.push(top_pile);
    }
    top_crates.to_string()
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle, puzzle_2};

    #[test]
    fn it_works() {
        let crates = vec![String::from("ZN"), String::from("MCD"), String::from("P")];

        let file_path = get_file_path(String::from("input.test.txt"));
        let result = puzzle(file_path, crates);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn it_works_with_whole_input() {
        let crates = vec![
            String::from("NDMQBPZ"),
            String::from("CLZQMDHV"),
            String::from("QHRDVFZG"),
            String::from("HGDFN"),
            String::from("NFQ"),
            String::from("DQVZFBT"),
            String::from("QMTZDVSH"),
            String::from("MGFPNQ"),
            String::from("BWRM"),
        ];

        let file_path = get_file_path(String::from("input.txt"));
        let result = puzzle(file_path, crates);
        assert_eq!(result, "QGTHFZBHV");
    }

    #[test]
    fn it_works_with_puzzle_2() {
        let crates = vec![String::from("ZN"), String::from("MCD"), String::from("P")];

        let file_path = get_file_path(String::from("input.test.txt"));
        let result = puzzle_2(file_path, crates);
        assert_eq!(result, "MCD");
    }

    #[test]
    fn it_works_with_whole_input_puzzle_2() {
        let crates = vec![
            String::from("NDMQBPZ"),
            String::from("CLZQMDHV"),
            String::from("QHRDVFZG"),
            String::from("HGDFN"),
            String::from("NFQ"),
            String::from("DQVZFBT"),
            String::from("QMTZDVSH"),
            String::from("MGFPNQ"),
            String::from("BWRM"),
        ];

        let file_path = get_file_path(String::from("input.txt"));
        let result = puzzle_2(file_path, crates);
        assert_eq!(result, "MGDMPSZTM");
    }
}
