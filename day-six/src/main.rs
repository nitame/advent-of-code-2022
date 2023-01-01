use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, io};

fn main() {
    println!("Advent of code day 6");

    let file_path = get_file_path(String::from("input.txt"));
    let result = puzzle_2(file_path);

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

fn puzzle(file_path: PathBuf) -> i32 {
    let mut result = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                //puzzle logic
                for i in 0..line.len() {
                    let packet = &line[i..i + 4];
                    let mut group_by_letter = HashSet::new();
                    for letter in packet.chars() {
                        group_by_letter.insert(letter);
                    }
                    if packet.len() == group_by_letter.len() {
                        result = i as i32;
                        break;
                    }
                }
            }
        }
    }
    result + 4 as i32
}

fn puzzle_2(file_path: PathBuf) -> i32 {
    let mut result = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                //puzzle logic
                for i in 0..line.len() {
                    let packet = &line[i..i + 14];
                    let mut group_by_letter = HashSet::new();
                    for letter in packet.chars() {
                        group_by_letter.insert(letter);
                    }
                    if packet.len() == group_by_letter.len() {
                        result = i as i32;
                        break;
                    }
                }
            }
        }
    }
    result + 14 as i32
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle, puzzle_2};

    #[test]
    fn it_works_with_input_1() {
        let file_path = get_file_path(String::from("input-1.test.txt"));

        let result = puzzle(file_path);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_works_with_input_2() {
        let file_path = get_file_path(String::from("input-2.test.txt"));

        let result = puzzle(file_path);
        assert_eq!(result, 5);
    }

    #[test]
    fn it_works_with_input_3() {
        let file_path = get_file_path(String::from("input-3.test.txt"));

        let result = puzzle(file_path);
        assert_eq!(result, 6);
    }

    #[test]
    fn it_works_with_input_4() {
        let file_path = get_file_path(String::from("input-4.test.txt"));

        let result = puzzle(file_path);
        assert_eq!(result, 10);
    }

    #[test]
    fn it_works_with_input_5() {
        let file_path = get_file_path(String::from("input-5.test.txt"));

        let result = puzzle(file_path);
        assert_eq!(result, 11);
    }

    #[test]
    fn it_works_with_whole_input() {
        let file_path = get_file_path(String::from("input.txt"));

        let result = puzzle(file_path);
        assert_eq!(result, 1658);
    }

    #[test]
    fn it_works_with_input_1_for_puzzle_2() {
        let file_path = get_file_path(String::from("input-1.test.txt"));

        let result = puzzle_2(file_path);
        assert_eq!(result, 19);
    }

    #[test]
    fn it_works_with_input_2_for_puzzle_2() {
        let file_path = get_file_path(String::from("input-2.test.txt"));

        let result = puzzle_2(file_path);
        assert_eq!(result, 23);
    }

    #[test]
    fn it_works_with_input_3_for_puzzle_2() {
        let file_path = get_file_path(String::from("input-3.test.txt"));

        let result = puzzle_2(file_path);
        assert_eq!(result, 23);
    }

    #[test]
    fn it_works_with_input_4_for_puzzle_2() {
        let file_path = get_file_path(String::from("input-4.test.txt"));

        let result = puzzle_2(file_path);
        assert_eq!(result, 29);
    }

    #[test]
    fn it_works_with_input_5_for_puzzle_2() {
        let file_path = get_file_path(String::from("input-5.test.txt"));

        let result = puzzle_2(file_path);
        assert_eq!(result, 26);
    }

    #[test]
    fn it_works_with_whole_input_for_puzzle_2() {
        let file_path = get_file_path(String::from("input.txt"));

        let result = puzzle_2(file_path);
        assert_eq!(result, 2260);
    }
}
