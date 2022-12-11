use std::fs::File;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::{env, io};

fn main() {
    println!("Advent of code day 10");

    let file_path = get_file_path(String::from("input.txt"));
    let mut rules = extract_rules(file_path);

    game(&mut rules);

    let result = calculate_monkey_business(rules);

    println!("result => {}", result);
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

fn extract_rules(file_path: PathBuf) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut position = 0;
    let mut monkey = Monkey::init();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                if line == "" {
                    monkeys.push(monkey);
                    monkey = Monkey::init();
                    position = position + 1;
                    continue;
                }

                let instructions: Vec<_> = line.split(':').collect();
                let left_part = instructions[0].trim();
                let right_part = instructions[1];
                match left_part {
                    "Starting items" => {
                        monkey.items = right_part
                            .split(',')
                            .map(|item| item.trim().parse().unwrap())
                            .collect()
                    }
                    "Operation" => {
                        monkey.operation = convert_to_operation(right_part.to_string());
                    }
                    "Test" => {
                        monkey.test = right_part
                            .split('y')
                            .last()
                            .unwrap()
                            .trim()
                            .parse()
                            .unwrap()
                    }
                    "If true" => {
                        monkey.if_true = right_part
                            .split('y')
                            .last()
                            .unwrap()
                            .trim()
                            .parse()
                            .unwrap()
                    }
                    "If false" => {
                        monkey.if_false = right_part
                            .split('y')
                            .last()
                            .unwrap()
                            .trim()
                            .parse()
                            .unwrap()
                    }

                    _ => monkey.position = position,
                }
            }
        }
    }
    monkeys
}

#[derive(Debug, Eq, PartialEq)]
struct Monkey {
    position: i32,
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    if_true: i32,
    if_false: i32,
    inspected_items: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct Operation {
    lhs: String,
    operator: String,
    rhs: String,
}

impl Monkey {
    fn init() -> Self {
        Monkey {
            position: 0,
            items: vec![],
            operation: Operation {
                lhs: "".to_string(),
                operator: "".to_string(),
                rhs: "".to_string(),
            },
            test: 0,
            if_true: 0,
            if_false: 0,
            inspected_items: 0,
        }
    }
}

fn convert_to_operation(input: String) -> Operation {
    let op: String = input.split("=").skip(1).collect();
    let operation: Vec<_> = op.split_whitespace().collect();
    let (a, b, c) = match &operation[..] {
        &[first, second, third, ..] => (first, second, third),
        _ => unreachable!(),
    };

    Operation {
        lhs: a.to_string(),
        operator: b.to_string(),
        rhs: c.to_string(),
    }
}

fn game_turn(pos: usize, monkeys: &mut Vec<Monkey>) {
    for item in monkeys[pos].items.clone() {
        let mut worry_level = apply_operation(item, &monkeys[pos].operation);
        worry_level = worry_level / 3;
        if worry_level % monkeys[pos].test == 0 {
            let position = monkeys[pos].if_true as usize;
            monkeys[position].items.push(worry_level);
        } else {
            let position = monkeys[pos].if_false as usize;
            monkeys[position].items.push(worry_level);
        }
    }
    monkeys[pos].inspected_items = monkeys[pos].inspected_items + monkeys[pos].items.len() as i32;
    monkeys[pos].items = vec![];
}

fn apply_operation(value: i32, operation: &Operation) -> i32 {
    let a = value;
    let b: i32;
    if operation.rhs == "old" {
        b = value;
    } else {
        b = operation.rhs.parse().unwrap()
    }
    match operation.operator.as_str() {
        "*" => a * b,
        "+" => a + b,
        "-" => a - b, // fixme: not needed operator
        _ => panic!("Unknown operator at operation operator"),
    }
}

fn game(monkeys: &mut Vec<Monkey>) {
    let number_of_monkeys = monkeys.len();
    for _round in 0..20 {
        for i in 0..number_of_monkeys {
            game_turn(i, monkeys);
        }
    }
}

fn calculate_monkey_business(monkeys: Vec<Monkey>) -> i32 {
    let mut inspected_items_by_monkey: Vec<_> = monkeys
        .iter()
        .map(|monkey| monkey.inspected_items)
        .collect();
    inspected_items_by_monkey.sort_by(|a, b| b.cmp(a));
    let hiqhest_monkey_business: Vec<_> = inspected_items_by_monkey.iter().take(2).collect();
    hiqhest_monkey_business[0] * hiqhest_monkey_business[1]
}

#[cfg(test)]
mod tests {
    use crate::{
        calculate_monkey_business, convert_to_operation, extract_rules, game, get_file_path,
        Monkey, Operation,
    };

    #[test]
    fn it_should_convert_operation() {
        // deprecated
        let input = String::from(" new = old * 19");
        let expected = Operation {
            lhs: "old".to_string(),
            operator: "*".to_string(),
            rhs: "19".to_string(),
        };
        let result = convert_to_operation(input);
        assert_eq!(result.lhs, expected.lhs);
        assert_eq!(result.operator, expected.operator);
        assert_eq!(result.rhs, expected.rhs);
    }

    #[test]
    fn it_should_work_with_whole_input() {
        let file_path = get_file_path(String::from("input.txt"));
        let mut rules = extract_rules(file_path);

        game(&mut rules);

        let result = calculate_monkey_business(rules);
        assert_eq!(result, 50616);
    }

    #[test]
    fn it_should_extract_rules() {
        let expected_rules = vec![
            Monkey {
                position: 0,
                items: vec![79, 98],
                operation: Operation {
                    lhs: "old".to_string(),
                    operator: "*".to_string(),
                    rhs: "19".to_string(),
                },
                test: 23,
                if_true: 2,
                if_false: 3,
                inspected_items: 0,
            },
            Monkey {
                position: 1,
                items: vec![54, 65, 75, 74],
                operation: Operation {
                    lhs: "old".to_string(),
                    operator: "+".to_string(),
                    rhs: "6".to_string(),
                },
                test: 19,
                if_true: 2,
                if_false: 0,
                inspected_items: 0,
            },
            Monkey {
                position: 2,
                items: vec![79, 60, 97],
                operation: Operation {
                    lhs: "old".to_string(),
                    operator: "*".to_string(),
                    rhs: "old".to_string(),
                },
                test: 13,
                if_true: 1,
                if_false: 3,
                inspected_items: 0,
            },
            Monkey {
                position: 3,
                items: vec![74],
                operation: Operation {
                    lhs: "old".to_string(),
                    operator: "+".to_string(),
                    rhs: "3".to_string(),
                },
                test: 17,
                if_true: 0,
                if_false: 1,
                inspected_items: 0,
            },
        ];
        let file_path = get_file_path(String::from("input.test.txt"));
        let rules = extract_rules(file_path);
        assert_eq!(rules, expected_rules);
    }
}
