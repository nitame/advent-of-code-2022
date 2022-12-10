use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, io};

fn main() {
    println!("Advent of code day 10");

    let file_path = get_file_path(String::from("input.txt"));
    let result = puzzle_2(file_path);

    println!("result -> {:?}", result);
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
    let mut program_output = vec![1];
    let mut value = 1;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                //puzzle logic
                let instructions: Vec<_> = line.split_whitespace().collect();
                if instructions[0] == "addx" {
                    let addx = instructions[1].parse::<i32>().unwrap();
                    let new_value = value + addx;
                    program_output.push(value);
                    program_output.push(new_value);
                    value = new_value;
                }
                if instructions[0] == "noop" {
                    program_output.push(value);
                }
            }
        }
    }
    extract_strenght_signal(program_output)
}

fn puzzle_2(file_path: PathBuf) -> Vec<String> {
    let mut program_output = vec![1];
    let mut value = 1;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                //puzzle logic
                let instructions: Vec<_> = line.split_whitespace().collect();
                if instructions[0] == "addx" {
                    let addx = instructions[1].parse::<i32>().unwrap();
                    let new_value = value + addx;
                    program_output.push(value);
                    program_output.push(new_value);
                    value = new_value;
                }
                if instructions[0] == "noop" {
                    program_output.push(value);
                }
            }
        }
    }
    display_screen(program_output)
}

fn display_screen(program: Vec<i32>) -> Vec<String> {
    let mut screen: Vec<String> = vec![];
    for i in 0..6 {
        let mut line_screen = String::new();
        for j in 0..40 {
            let program_cursor = 40 * i + j;
            let stripe_start = program[program_cursor] - 1;
            let stripe_end = program[program_cursor] + 1;
            if j as i32 >= stripe_start && j as i32 <= stripe_end {
                line_screen.push('#');
            } else {
                line_screen.push('.');
            }
        }
        println!("{}", line_screen);
        screen.push(line_screen);
    }
    screen
}

fn extract_strenght_signal(sequence: Vec<i32>) -> i32 {
    let mut signal_strenght_list = vec![];
    for i in (20..sequence.len()).step_by(40) {
        signal_strenght_list.push((i as i32, sequence[i - 1]))
    }
    signal_strenght_list
        .iter()
        .map(|item| {
            let (coef, val) = item;
            coef * val
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle, puzzle_2};

    #[test]
    fn it_works_with_test_input() {
        let file_path = get_file_path(String::from("input.test.txt"));

        let result = puzzle(file_path);
        assert_eq!(result, 13140);
    }

    #[test]
    fn it_works_with_whole_input() {
        let file_path = get_file_path(String::from("input.txt"));

        let result = puzzle(file_path);
        assert_eq!(result, 12640);
    }

    #[test]
    fn it_works_with_test_input_puzzle_2() {
        let file_path = get_file_path(String::from("input.test.txt"));
        let expected = vec![
            String::from("##..##..##..##..##..##..##..##..##..##.."),
            String::from("###...###...###...###...###...###...###."),
            String::from("####....####....####....####....####...."),
            String::from("#####.....#####.....#####.....#####....."),
            String::from("######......######......######......####"),
            String::from("#######.......#######.......#######....."),
        ];

        let result = puzzle_2(file_path);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_works_with_whole_input_puzzle_2() {
        let file_path = get_file_path(String::from("input.txt"));
        let expected = vec![
            String::from("####.#..#.###..####.#....###....##.###.."),
            String::from("#....#..#.#..#....#.#....#..#....#.#..#."),
            String::from("###..####.###....#..#....#..#....#.#..#."),
            String::from("#....#..#.#..#..#...#....###.....#.###.."),
            String::from("#....#..#.#..#.#....#....#.#..#..#.#.#.."),
            String::from("####.#..#.###..####.####.#..#..##..#..#."),
        ];

        let result = puzzle_2(file_path);
        assert_eq!(result, expected);
    }
}
