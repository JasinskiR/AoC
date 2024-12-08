use code_timing_macros::time_snippet;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DAY: usize = 8;

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
    let mut grid = Vec::new();
    for line in lines {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut unique_antinodes: HashSet<(usize, usize)> = HashSet::new();
    let rows = grid.len();
    let cols = grid[0].len();

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert_with(Vec::new).push((i, j));
            }
        }
    }

    for (_freq, positions) in antennas.iter() {
        for (i, &a1) in positions.iter().enumerate() {
            for (j, &a2) in positions.iter().enumerate() {
                if i != j {
                    let dx = a2.1 as i32 - a1.1 as i32;
                    let dy = a2.0 as i32 - a1.0 as i32;


                    let antinode1 = (
                        a1.0 as i32 - dy,
                        a1.1 as i32 - dx,
                    );

                    let antinode2 = (
                        a2.0 as i32 + dy,
                        a2.1 as i32 + dx,
                    );

                    if antinode1.0 >= 0 && antinode1.0 < rows as i32
                        && antinode1.1 >= 0 && antinode1.1 < cols as i32
                    {
                        unique_antinodes.insert((antinode1.0 as usize, antinode1.1 as usize));
                    }
                    if antinode2.0 >= 0 && antinode2.0 < rows as i32
                        && antinode2.1 >= 0 && antinode2.1 < cols as i32
                    {
                        unique_antinodes.insert((antinode2.0 as usize, antinode2.1 as usize));
                    }
                }
            }
        }
    }

    Ok(unique_antinodes.len() as i32)
}

fn part2(lines: &[String]) -> Result<i32, io::Error> {
    let mut grid = Vec::new();
    for line in lines {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut unique_antinodes: HashSet<(usize, usize)> = HashSet::new();
    let rows = grid.len();
    let cols = grid[0].len();

    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert_with(Vec::new).push((i, j));
            }
        }
    }

    // Process each frequency
    for (_freq, positions) in antennas.iter() {
        for (i, &a1) in positions.iter().enumerate() {
            for (j, &a2) in positions.iter().enumerate() {
                if i != j {
                    // Calculate the direction vector (dx, dy)
                    let dx = a2.1 as i32 - a1.1 as i32;
                    let dy = a2.0 as i32 - a1.0 as i32;
                    let vec = (dx, dy);

                    // Antinode 1 (towards a1)
                    let mut antinode1 = (
                        a1.0 as i32 - dy,
                        a1.1 as i32 - dx,
                    );
                    // Antinode 2 (beyond a2)
                    let mut antinode2 = (
                        a2.0 as i32 + dy,
                        a2.1 as i32 + dx,
                    );

                    while antinode1.0 >= 0 && antinode1.0 < rows as i32
                        && antinode1.1 >= 0 && antinode1.1 < cols as i32
                    {
                        unique_antinodes.insert((antinode1.0 as usize, antinode1.1 as usize));
                        antinode1.0 -= dy;
                        antinode1.1 -= dx;
                    }
                    while antinode2.0 >= 0 && antinode2.0 < rows as i32
                        && antinode2.1 >= 0 && antinode2.1 < cols as i32
                    {
                        unique_antinodes.insert((antinode2.0 as usize, antinode2.1 as usize));
                        antinode2.0 += dy;
                        antinode2.1 += dx;
                    }
                }
            }
        }

        for &antenna in positions {
            unique_antinodes.insert(antenna);
        }
    }

    Ok(unique_antinodes.len() as i32)
}