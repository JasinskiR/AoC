use code_timing_macros::time_snippet;
use std::collections::{HashMap};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DAY: usize = 1;

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

fn solve(lines: &[String]) -> Result<(i32, i32), io::Error> {
    let part1_result = part1(lines)?;
    let part2_result = part2(lines)?;

    println!("Part 1 result: {}", part1_result);
    println!("Part 2 result: {}", part2_result);

    Ok((part1_result, part2_result))
}

fn part1(lines: &[String]) -> Result<i32, io::Error> {
    let mut vector_left: Vec<i32> = Vec::new();
    let mut vector_right: Vec<i32> = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            let num1 = parts[0].parse::<i32>().unwrap();
            let num2 = parts[1].parse::<i32>().unwrap();
            vector_left.push(num1);
            vector_right.push(num2);
        }
    }

    vector_left.sort();
    vector_right.sort();

    for i in (0..vector_left.len()).rev() {
        if let Some(index) = vector_right.iter().position(|&x| x == vector_left[i]) {
            vector_right.remove(index);
            vector_left.remove(i);
        }
    }

    let total_sum: i32 = vector_left
        .iter()
        .zip(vector_right.iter())
        .map(|(&left, &right)| (left - right).abs())
        .sum();

    Ok(total_sum)
}

fn part2(lines: &[String]) -> Result<i32, io::Error> {
    let mut hash_map_left: HashMap<i32, i32> = HashMap::new();
    let mut hash_map_right: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() >= 2 {
            let num1 = parts[0].parse::<i32>().unwrap();
            let num2 = parts[1].parse::<i32>().unwrap();
            *hash_map_left.entry(num1).or_insert(0) += 1;
            *hash_map_right.entry(num2).or_insert(0) += 1;
        }
    }

    let mut total_sum = 0;
    for (key, &left_count) in &hash_map_left {
        if let Some(&right_count) = hash_map_right.get(key) {
            total_sum += left_count * right_count * key;
        }
    }

    Ok(total_sum)
}
