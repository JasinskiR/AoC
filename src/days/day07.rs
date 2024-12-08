use code_timing_macros::time_snippet;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DAY: usize = 7;

fn get_file_path(day: usize, is_test: bool) -> String {
    let suffix = if is_test { "_test" } else { "" };
    format!("inputs/day{:02}{}.txt", day, suffix)
}

pub fn run(mode: bool) {
    let input_path = get_file_path(DAY, mode);
    if mode {
        println!("Using test file: {}", input_path);
    } else {
        println!("Using input file: {}", input_path);
    }

    let input_file = File::open(&input_path).expect("Failed to open input file");
    let input_reader = BufReader::new(input_file);

    // Collect all lines into a vector
    let lines: Vec<String> = input_reader.lines().filter_map(Result::ok).collect();

    let result = time_snippet!(solve(&lines));
    println!("Result = {:?}", result);
}

fn solve(lines: &[String]) -> Result<(i64, i64), io::Error> {
    let part1_result = part1(lines)?;
    let part2_result = part2(lines)?;

    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);

    Ok((part1_result, part2_result))
}

fn parse_input(lines: &[String]) -> HashMap<i64, Vec<i32>> {
    let mut result = HashMap::new();

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        if let Some((key, values)) = line.split_once(':') {
            if let Ok(key_parsed) = key.trim().parse::<i64>() {
                let parsed_values: Vec<i32> = values
                    .split_whitespace() 
                    .filter_map(|x| x.trim().parse::<i32>().ok())
                    .collect();

                result.insert(key_parsed, parsed_values);
            }
        }
    }

    result
}

fn evaluate_expression(numbers: &Vec<i32>, operators: &[char]) -> i64 {
    let mut result = numbers[0] as i64;
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '+' => result += numbers[i + 1] as i64,
            '*' => result *= numbers[i + 1] as i64,
            '|' => {
                result = (result.to_string() + &numbers[i + 1].to_string())
                    .parse::<i64>()
                    .unwrap()
            }
            _ => panic!("Unsupported operator"),
        }
    }
    result
}

fn generate_operators(n: usize) -> Vec<Vec<char>> {
    let mut results = Vec::new();
    let operators = vec!['+', '*'];
    let total_combinations = 1 << (n - 1);

    for i in 0..total_combinations {
        let mut combination = Vec::new();
        for j in 0..(n - 1) {
            if (i & (1 << j)) != 0 {
                combination.push(operators[0]);
            } else {
                combination.push(operators[1]);
            }
        }
        results.push(combination);
    }
    results
}

fn part1(lines: &[String]) -> Result<i64, io::Error> {
    let equations = parse_input(lines);

    println!("Total equations: {}", equations.len());
    let mut total: i64 = 0;

    for (test_value, numbers) in equations {
        if numbers.len() < 2 {
            continue;
        }

        let operator_combinations = generate_operators(numbers.len());
        let mut found = false;

        for operators in operator_combinations {
            if evaluate_expression(&numbers, &operators) == test_value as i64 {
                found = true;
                break;
            }
        }

        if found {
            total += test_value as i64;
        }
    }
    Ok(total)
}

fn generate_operators_with_concat(n: usize) -> Vec<Vec<char>> {
    let mut results = Vec::new();
    let operators = vec!['+', '*', '|'];
    let total_combinations = operators.len().pow((n - 1) as u32);

    for i in 0..total_combinations {
        let mut combination = Vec::new();
        let mut value = i;

        for _ in 0..(n - 1) {
            combination.push(operators[value % operators.len()]);
            value /= operators.len();
        }
        results.push(combination);
    }
    results
}

fn part2(lines: &[String]) -> Result<i64, std::io::Error> {
    let equations = parse_input(lines);
    let mut total: i64 = 0;

    for (test_value, numbers) in equations {
        if numbers.len() < 2 {
            continue;
        }

        let operator_combinations = generate_operators_with_concat(numbers.len());
        let mut found = false;

        for operators in operator_combinations {
            if evaluate_expression(&numbers, &operators) == test_value as i64 {
                found = true;
                break;
            }
        }

        if found {
            total += test_value as i64;
        }
    }
    Ok(total)
}
