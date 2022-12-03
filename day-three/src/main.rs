use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

fn main() {
    println!("Advent of code day 3");

    let total = puzzle();
    println!("{total}");
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn puzzle() -> i32 {
    let current_dir = env::current_dir().expect("Attempt to get current dir");
    println!("current dir {:?}", current_dir);

    let file_path = current_dir.join(Path::new("input.txt"));
    println!("file path: {:?}", file_path);

    let mut total = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                // logic puzzle
                total = total + get_priority(line);
            }
        }
    }
    total
}

fn get_priority(input: String) -> i32 {
    let part_1 = &input[0..(input.len() / 2)];
    let part_2 = &input[(input.len() / 2)..];
    let common_item = part_1
        .chars()
        .find(|item| part_2.contains(|part_2_item| part_2_item == item.to_owned()))
        .expect("Failed to retrieve common item");
    let priority_index = ALPHABET
        .find(common_item)
        .expect("Failed to retrieve priority index");
    priority_index as i32 + 1
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use crate::puzzle;

    #[test]
    fn it_works() {
        let result = puzzle();
        assert_eq!(result, 8109);
    }
}
