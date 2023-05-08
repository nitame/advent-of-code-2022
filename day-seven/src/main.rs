use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, io};

fn main() {
    println!("Advent of code day 7");

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

fn get_file_path(filename: String) -> PathBuf {
    let current_dir = env::current_dir().expect("Attempt to get current dir");
    current_dir.join(Path::new(filename.as_str()))
}

fn puzzle(file_path: PathBuf) -> u32 {
    let registry = build_regitry(file_path);
    let registry_grouped_by_directories = sum_size_by_parent_directories(&registry);
    registry_grouped_by_directories
        .into_values()
        .filter(|x| *x < 100_000)
        .sum()
}

fn build_regitry(file_path: PathBuf) -> HashMap<String, u32> {
    let mut registry: HashMap<String, u32> = HashMap::new();
    if let Ok(lines) = read_lines(file_path) {
        let mut current_path = Vec::new();
        for line in lines {
            if let Ok(line) = line {
                let elems: Vec<_> = line.split_whitespace().collect();
                match elems[..] {
                    ["$", "cd", ".."] => {
                        current_path.pop();
                    }
                    ["$", "cd", name] => {
                        current_path.push(name.to_string());
                        registry.insert(current_path.join(""), 0);
                    }
                    ["$", "ls"] => (),
                    ["dir", _] => (),
                    [size, _] => {
                        let path_string = current_path.join("");
                        let file_size = size.parse::<u32>().expect("Cannot parse size file");
                        registry
                            .entry(path_string)
                            .and_modify(|v| *v += file_size)
                            .or_insert(file_size);
                    }
                    _ => (),
                }
            }
        }
    }
    registry
}

fn sum_size_by_parent_directories(registry: &HashMap<String, u32>) -> HashMap<String, u32> {
    let mut summed_registry: HashMap<String, u32> = HashMap::new();
    for i in registry {
        let mut sum = 0;
        for j in registry {
            if j.0.starts_with(i.0) {
                sum += j.1
            }
        }
        summed_registry.insert(i.0.clone(), sum);
    }
    summed_registry
}

#[cfg(test)]
mod tests {
    use crate::{get_file_path, puzzle};

    #[test]
    fn it_works_with_input_test() {
        let file_path = get_file_path(String::from("input.test.txt"));

        let result = puzzle(file_path);
        assert_eq!(result, 95437);
    }

    #[test]
    fn it_works_with_input() {
        let file_path = get_file_path(String::from("input.txt"));

        let result = puzzle(file_path);
        assert_eq!(result, 1443806);
    }
}
