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
    registry.into_values().filter(|x| *x < 100_000).sum()
}

fn build_regitry(file_path: PathBuf) -> HashMap<Vec<String>, u32> {
    let mut registry: HashMap<Vec<String>, u32> = HashMap::new();
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
                        registry.insert(current_path.clone(), 0);
                    }
                    ["$", "ls"] => (),
                    ["dir", _] => (),
                    [size, _] => {
                        let file_size = size.parse::<u32>().expect("Cannot parse size file");
                        update_registry(&mut registry, &current_path, file_size);
                    }
                    _ => (),
                }
            }
        }
    }
    registry
}

fn update_registry(
    registry: &mut HashMap<Vec<String>, u32>,
    current_path: &Vec<String>,
    size: u32,
) {
    registry
        .entry(current_path.clone())
        .and_modify(|v| *v += size)
        .or_insert(size);
    if let Some((_, head)) = current_path.split_last() {
        update_registry(registry, &head.to_vec(), size);
    }
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
