use code_timing_macros::time_snippet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DAY: usize = 4;

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
    let word = "XMAS";
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let mut total = 0;

    fn matches(
        grid: &[Vec<char>],
        word: &str,
        start_row: usize,
        start_col: usize,
        delta_row: isize,
        delta_col: isize,
    ) -> bool {
        let word_len = word.len();
        let rows = grid.len() as isize;
        let cols = grid[0].len() as isize;

        for i in 0..word_len {
            let new_row = start_row as isize + i as isize * delta_row;
            let new_col = start_col as isize + i as isize * delta_col;

            if new_row < 0 || new_col < 0 || new_row >= rows || new_col >= cols {
                return false;
            }

            if grid[new_row as usize][new_col as usize] != word.chars().nth(i).unwrap() {
                return false;
            }
        }
        true
    }

    for row in 0..rows {
        for col in 0..cols {
            // Check all 8 possible directions
            let directions = [
                (0, 1),   // Right
                (0, -1),  // Left
                (1, 0),   // Down
                (-1, 0),  // Up
                (1, 1),   // Down-Right
                (1, -1),  // Down-Left
                (-1, 1),  // Up-Right
                (-1, -1), // Up-Left
            ];

            for &(delta_row, delta_col) in &directions {
                if matches(&grid, word, row, col, delta_row, delta_col) {
                    total += 1;
                }
            }
        }
    }

    Ok(total)
}

fn part2(lines: &[String]) -> Result<i32, std::io::Error> {
    let word_as = "AS";
    let word_am = "AM";

    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = if rows > 0 { grid[0].len() } else { 0 };
    let mut total = 0;

    fn matches(
        grid: &[Vec<char>],
        word: &str,
        start_row: usize,
        start_col: usize,
        delta_row: isize,
        delta_col: isize,
    ) -> bool {
        let word_len = word.len();
        let rows = grid.len() as isize;
        let cols = grid[0].len() as isize;

        for i in 0..word_len {
            let new_row = start_row as isize + i as isize * delta_row;
            let new_col = start_col as isize + i as isize * delta_col;

            if new_row < 0 || new_col < 0 || new_row >= rows || new_col >= cols {
                return false;
            }

            if grid[new_row as usize][new_col as usize] != word.chars().nth(i).unwrap() {
                return false;
            }
        }
        true
    }

    for row in 0..rows {
        for col in 0..cols {
            let directions = [
                (1, 1),   // Down-Right
                (1, -1),  // Down-Left
                (-1, 1),  // Up-Right
                (-1, -1), // Up-Left
            ];
            let mut x_shape_mas = 0;

            for (i, &(delta_row, delta_col)) in directions.iter().enumerate() {
                if matches(&grid, &word_am, row, col, delta_row, delta_col) {
                    let (delta_row_as, delta_col_as) = directions[directions.len() - 1 - i];
                    if matches(&grid, &word_as, row, col, delta_row_as, delta_col_as) {
                        x_shape_mas += 1;
                    }
                }
            }
            if x_shape_mas == 2 {
                total += 1;
            }
        }
    }

    Ok(total)
}
