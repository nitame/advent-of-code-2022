use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{env, io};

fn main() {
    println!("Advent of code day 3");

    let total_1 = puzzle();
    let total_2 = puzzle_2();
    println!("Puzzle 1 solution {total_1}");
    println!("Puzzle 2 solution {}", total_2);
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn puzzle() -> i32 {
    let current_dir = env::current_dir().expect("Attempt to get current dir");
    let file_path = current_dir.join(Path::new("input.txt"));

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

fn puzzle_2() -> u32 {
    let current_dir = env::current_dir().expect("Attempt to get current dir");
    let file_path = current_dir.join(Path::new("input.txt"));

    let mut rucksacks_grouped_by_3_elves: Vec<Vec<String>> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        let mut triple: Vec<String> = Vec::new();
        let mut index: usize = 0;
        for line in lines {
            if let Ok(line) = line {
                if index == 2 {
                    triple.push(line.clone());
                    rucksacks_grouped_by_3_elves.push(triple.clone());
                    triple.pop();
                    triple.pop();
                    triple.pop();
                    index = 0;
                } else {
                    triple.push(line);
                    index = index + 1
                }
            }
        }
    }

    let common_item_list = find_common_item_for_each_groups(rucksacks_grouped_by_3_elves);
    let priorities = convert_to_priorities(common_item_list);
    priorities.iter().sum()
}

fn find_common_item_for_each_groups(input: Vec<Vec<String>>) -> Vec<char> {
    let mut common_items_list: Vec<char> = Vec::new();
    for group in input {
        let elf_1 = group.get(0).expect("Cannot get elf 1 items");
        let elf_2 = group.get(1).expect("Cannot get elf 2 items");
        let elf_3 = group.get(2).expect("Cannot get elf 3 items");
        let common = elf_1
            .chars()
            .find(|c| {
                elf_2.contains(|cc| cc == c.to_owned()) && elf_3.contains(|ccc| ccc == c.to_owned())
            })
            .expect("No common item found between elf 1, elf 2, elf 3");
        common_items_list.push(common);
    }
    common_items_list
}

fn convert_to_priorities(input: Vec<char>) -> Vec<u32> {
    input
        .iter()
        .map(|c| {
            ALPHABET
                .find(*c)
                .expect("Failed to retrieve priority index")
        })
        .map(|value| value as u32 + 1)
        .collect()
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
    use crate::{puzzle, puzzle_2};

    #[test]
    fn it_works_puzzle_1() {
        let result = puzzle();
        assert_eq!(result, 8109);
    }

    #[test]
    fn it_works_puzzle_2() {
        let result = puzzle_2();
        assert_eq!(result, 2738);
    }
}
