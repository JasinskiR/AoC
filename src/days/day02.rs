use code_timing_macros::time_snippet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DAY: usize = 2;

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

fn part1(lines: &[String]) -> Result<i32, std::io::Error> {
    // println!("Lines are: {:?}", lines);

    #[derive(PartialEq, Debug)]
    enum Mode {
        Ascending,
        Descending,
    }

    let mut counts = 0;

    for line in lines {
        let parts: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        if parts.len() < 2 {
            continue;
        }

        let mode = if parts[0] > parts[1] {
            Mode::Descending
        } else if parts[0] < parts[1] {
            Mode::Ascending
        } else {
            continue;
        };

        let mut is_valid = true;

        for i in 1..parts.len() {
            let prev = parts[i - 1];
            let curr = parts[i];
            let diff = (curr - prev).abs();

            if !(1..=3).contains(&diff)
                || (mode == Mode::Descending && curr >= prev)
                || (mode == Mode::Ascending && curr <= prev)
            {
                // println!("Break on line: {}", line);
                is_valid = false;
                break;
            }
        }

        if is_valid {
            counts += 1;
        }
    }

    Ok(counts)
}

fn part2(lines: &[String]) -> Result<i32, std::io::Error> {
    #[derive(PartialEq, Debug)]
    enum Mode {
        Ascending,
        Descending,
    }

    let mut counts = 0;

    for line in lines {
        let parts: Vec<i32> = line
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();

        if parts.len() < 2 {
            continue;
        }

        let is_safe = |levels: &[i32]| -> bool {
            let mode = if levels[0] < levels[1] {
                Mode::Ascending
            } else if levels[0] > levels[1] {
                Mode::Descending
            } else {
                return false;
            };

            for i in 1..levels.len() {
                let diff = (levels[i] - levels[i - 1]).abs();
                if !(1..=3).contains(&diff)
                    || (mode == Mode::Ascending && levels[i] <= levels[i - 1])
                    || (mode == Mode::Descending && levels[i] >= levels[i - 1])
                {
                    return false;
                }
            }
            true
        };

        if is_safe(&parts) {
            counts += 1;
            continue;
        }

        let mut dampened_safe = false;
        for i in 0..parts.len() {
            let mut modified = parts.clone();
            modified.remove(i);
            if is_safe(&modified) {
                dampened_safe = true;
                break;
            }
        }

        if dampened_safe {
            counts += 1;
        }
    }

    Ok(counts)
}