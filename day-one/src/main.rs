use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, io};

fn main() {
    println!("Advent of code day 1");
    let file_path = get_file_path();

    let max_calories_elf = puzzle_1(file_path.clone());
    println!("max_calories_elf: {}", max_calories_elf);

    let top_3_elves_total = puzzle_2(file_path);
    println!("top_3_elves_total: {}", top_3_elves_total);
}

fn get_file_path() -> PathBuf {
    let current_dir = env::current_dir().expect("Attempt to get current dir");
    current_dir.join(Path::new("input.txt"))
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn puzzle_1(file: PathBuf) -> i32 {
    let mut max_calories_elf = 0;
    if let Ok(lines) = read_lines(file) {
        let mut sum_elf_items = 0;
        for line in lines {
            if let Ok(elf_calory) = line {
                if elf_calory != "" {
                    let num: i32 = elf_calory.parse().unwrap();
                    sum_elf_items = sum_elf_items + num;
                } else {
                    if sum_elf_items > max_calories_elf {
                        max_calories_elf = sum_elf_items;
                    }
                    sum_elf_items = 0;
                }
            }
        }
    }
    max_calories_elf
}

fn puzzle_2(file: PathBuf) -> i32 {
    let mut elves_calories = Vec::new();
    if let Ok(lines) = read_lines(file) {
        let mut sum_elf_items = 0;
        for line in lines {
            if let Ok(elf_calory) = line {
                if elf_calory != "" {
                    let num: i32 = elf_calory.parse().unwrap();
                    sum_elf_items = sum_elf_items + num;
                } else {
                    elves_calories.push(sum_elf_items);
                    sum_elf_items = 0;
                }
            }
        }
    }
    elves_calories.sort_by(|a, b| b.cmp(a));
    elves_calories.iter().take(3).sum::<i32>()
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle_1, puzzle_2};

    #[test]
    fn it_works() {
        let file_path = get_file_path();
        let result = puzzle_1(file_path);
        assert_eq!(result, 69528);
    }

    #[test]
    fn it_works_with_puzzle_2() {
        let file_path = get_file_path();
        let result = puzzle_2(file_path);
        assert_eq!(result, 206152);
    }
}
