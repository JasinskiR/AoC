use code_timing_macros::time_snippet;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DAY: usize = 3;

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
    // print!("String: {}", lines[0].as_str());
    let re = Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)").unwrap();

    let mut total = 0;
    for line in lines {
        let pairs: Vec<(&str, &str)> = re
            .captures_iter(line.as_str())
            .map(|caps| {
                let left = caps.name("left").unwrap().as_str();
                let right = caps.name("right").unwrap().as_str();
                (left, right)
            })
            .collect();

        for (left, right) in pairs {
            total += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
        }
    }

    Ok(total)
}

fn part2(lines: &[String]) -> Result<i32, std::io::Error> {
    let mut mode = true;
    // print!("String: {}\n", lines[0].as_str());
    let re = Regex::new(r"mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)").unwrap();

    let mut total = 0;

    for line in lines {
        let parts_by_dont: Vec<&str> = line.split("don't()").collect();

        for (i, part) in parts_by_dont.iter().enumerate() {
            // print!("Part don't: {}\n", part);

            if i == 0 && mode {
                let pairs: Vec<(&str, &str)> = re
                    .captures_iter(part)
                    .map(|caps| {
                        let left = caps.name("left").unwrap().as_str();
                        let right = caps.name("right").unwrap().as_str();
                        (left, right)
                    })
                    .collect();
                for (left, right) in pairs {
                    let result = left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
                    total += result;
                }
            }
            mode = false;

            let parts_by_do: Vec<&str> = part.split("do()").collect();
            for (j, do_part) in parts_by_do.iter().enumerate() {
                // print!("Part do: {}\n", do_part);

                if j > 0 {
                    let pairs: Vec<(&str, &str)> = re
                        .captures_iter(do_part)
                        .map(|caps| {
                            let left = caps.name("left").unwrap().as_str();
                            let right = caps.name("right").unwrap().as_str();
                            (left, right)
                        })
                        .collect();
                    for (left, right) in &pairs {
                        total += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
                    }
                    mode = true;
                }
            }
        }
    }

    Ok(total)
}
