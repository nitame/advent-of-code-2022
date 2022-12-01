use std::{env, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

fn main() {
    println!("Advent of code day 1");

    let current_dir = env::current_dir().expect("Attempt to get current dir");
    println!("current dir {:?}", current_dir);

    let file_path = current_dir.join(Path::new("input.txt"));
    println!("file path: {:?}", file_path);

    let mut max_calories_elf = 0;
    if let Ok(lines) = read_lines(file_path) {
        let mut sum_elf_items = 0;
        for line in lines {
            if let Ok(elf_calory) = line {
                if elf_calory != "" {
                    println!("no empty line, convert and sum");
                    //convert and sum
                    let num: i32 = elf_calory.parse().unwrap();
                    sum_elf_items = sum_elf_items + num;
                } else {
                    println!("empty line, reset counter");
                    // compare with max
                    if sum_elf_items > max_calories_elf {
                        max_calories_elf = sum_elf_items;
                    }
                    sum_elf_items = 0;
                }
            }
            println!("sum_elf_items at each line: {}", sum_elf_items);
        }
    }

    println!("max_calories_elf: {}", max_calories_elf);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}