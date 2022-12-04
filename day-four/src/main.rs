use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, io};

fn main() {
    println!("Advent of code day 4");

    let file_path = get_file_path(String::from("input.txt"));
    let result = puzzle(file_path);

    println!("result -> {result}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn puzzle(file_path: PathBuf) -> i32 {
    let mut total = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                // logic puzzle
                let split_elves_section: Vec<_> = line.split(",").collect();
                let left_elf_section_bounds: Vec<_> = split_elves_section[0].split("-").collect();
                let right_elf_section_bounds: Vec<_> = split_elves_section[1].split("-").collect();

                let is_left_contains_right =
                    section_contains(&left_elf_section_bounds, &right_elf_section_bounds);
                let is_right_contains_left =
                    section_contains(&right_elf_section_bounds, &left_elf_section_bounds);

                if is_left_contains_right || is_right_contains_left {
                    total = total + 1
                }
            }
        }
    }
    total
}

fn get_file_path(filename: String) -> PathBuf {
    let current_dir = env::current_dir().expect("Attempt to get current dir");
    current_dir.join(Path::new(filename.as_str()))
}

fn section_contains(left_section: &Vec<&str>, right_section: &Vec<&str>) -> bool {
    let left_start_bound: i32 = left_section[0].parse().unwrap();
    let left_end_bound: i32 = left_section[1].parse().unwrap();
    let right_start_bound: i32 = right_section[0].parse().unwrap();
    let right_end_bound: i32 = right_section[1].parse().unwrap();

    left_start_bound <= right_start_bound && left_end_bound >= right_end_bound
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle};

    #[test]
    fn it_works() {
        let file_path = get_file_path(String::from("input.test.txt"));
        let result = puzzle(file_path);
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_with_whole_input() {
        let file_path = get_file_path(String::from("input.txt"));
        let result = puzzle(file_path);
        assert_eq!(result, 490);
    }
}
