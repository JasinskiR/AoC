use code_timing_macros::time_snippet;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DAY: usize = 6;

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
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let mut guard_pos = (0, 0);
    let mut guard_dir = '^';

    // Populate grid HashMap and locate the guard's position
    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid.insert((i, j), ch);
            if "^>v<".contains(ch) {
                guard_pos = (i, j);
                guard_dir = ch;
            }
        }
    }

    // Remove the initial guard position from the grid
    grid.insert(guard_pos, '.');

    // Direction vectors and turning rules
    let directions: HashMap<char, (isize, isize)> =
        HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);
    let turn_right: HashMap<char, char> =
        HashMap::from([('^', '>'), ('>', 'v'), ('v', '<'), ('<', '^')]);

    // HashSet to track unique visited positions
    let mut visited = HashSet::new();
    visited.insert(guard_pos);

    // Simulate guard movement
    loop {
        // Collect all obstacles in the same row or column as the guard
        let relevant_obstacles: Vec<(usize, usize)> = match guard_dir {
            '^' | 'v' => grid
                .iter()
                .filter_map(|(&(x, y), &c)| {
                    if c == '#' && y == guard_pos.1 {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect(),
            '<' | '>' => grid
                .iter()
                .filter_map(|(&(x, y), &c)| {
                    if c == '#' && x == guard_pos.0 {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect(),
            _ => vec![],
        };
        // print!("Relevant obstacles: {:?}  ", relevant_obstacles);

        // Find the closest obstacle in the relevant direction
        let next_obstacle = match guard_dir {
            '^' => relevant_obstacles
                .into_iter()
                .filter(|&(x, _)| x < guard_pos.0)
                .max_by_key(|&(x, _)| x),
            'v' => relevant_obstacles
                .into_iter()
                .filter(|&(x, _)| x > guard_pos.0)
                .min_by_key(|&(x, _)| x),
            '<' => relevant_obstacles
                .into_iter()
                .filter(|&(_, y)| y < guard_pos.1)
                .max_by_key(|&(_, y)| y),
            '>' => relevant_obstacles
                .into_iter()
                .filter(|&(_, y)| y > guard_pos.1)
                .min_by_key(|&(_, y)| y),
            _ => None,
        };
        // print!("Next obstacle: {:?}  ", next_obstacle);

        if let Some(obstacle) = next_obstacle {
            // Move to the position right before the obstacle
            let distance = match guard_dir {
                '^' | 'v' => (obstacle.0 as isize - guard_pos.0 as isize).abs() as usize - 1,
                '<' | '>' => (obstacle.1 as isize - guard_pos.1 as isize).abs() as usize - 1,
                _ => 0,
            };

            for _ in 0..distance {
                let (dx, dy) = directions[&guard_dir];
                guard_pos = (
                    (guard_pos.0 as isize + dx) as usize,
                    (guard_pos.1 as isize + dy) as usize,
                );
                grid.insert(guard_pos, 'X');
                visited.insert(guard_pos);
            }
        } else {
            // If no obstacle, move until the grid edge
            loop {
                let (dx, dy) = directions[&guard_dir];
                let next_pos = (
                    (guard_pos.0 as isize + dx) as usize,
                    (guard_pos.1 as isize + dy) as usize,
                );

                if let Some('.') = grid.get(&next_pos) {
                    guard_pos = next_pos;
                    grid.insert(guard_pos, 'X');
                    visited.insert(guard_pos);
                } else {
                    break;
                }
            }
            break;
        }

        guard_dir = turn_right[&guard_dir];
        // print!("Guard position: {:?}\n", guard_pos);
    }

    Ok(visited.len() as i32)
}

fn part2(lines: &[String]) -> Result<i32, std::io::Error> {
    let mut grid: HashMap<(usize, usize), char> = HashMap::new();
    let mut guard_pos = (0, 0);
    let mut guard_dir = '^';

    // Populate grid HashMap and locate the guard's position
    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid.insert((i, j), ch);
            if "^>v<".contains(ch) {
                guard_pos = (i, j);
                guard_dir = ch;
            }
        }
    }

    // HashSet to track unique visited positions and directions
    let mut visited = HashSet::new();
    visited.insert((guard_pos, guard_dir));

    // Helper function to simulate the guard's movement with an added obstacle
    fn causes_loop(
        grid: &HashMap<(usize, usize), char>,
        start_pos: (usize, usize),
        start_dir: char,
    ) -> bool {
        let mut guard_pos = start_pos;
        let mut guard_dir = start_dir;
        let mut visited = HashSet::new();
        visited.insert((guard_pos, guard_dir));

        // Create a closure for the movement logic to avoid capturing dynamic environment
        let directions: HashMap<char, (isize, isize)> =
            HashMap::from([('^', (-1, 0)), ('>', (0, 1)), ('v', (1, 0)), ('<', (0, -1))]);
        let turn_right: HashMap<char, char> =
            HashMap::from([('^', '>'), ('>', 'v'), ('v', '<'), ('<', '^')]);

        // Run the loop for the guard's movement
        loop {
            // Access the directions and turn_right maps inside the loop.
            let (dx, dy) = directions[&guard_dir];
            let next_pos = (
                (guard_pos.0 as isize + dx) as usize,
                (guard_pos.1 as isize + dy) as usize,
            );

            // Check for loop detection: If the guard revisits the same position with the same direction
            if visited.contains(&(next_pos, guard_dir)) {
                return true; // A loop is detected
            }

            // Mark the position as visited with the current direction
            visited.insert((next_pos, guard_dir));

            // If the guard reaches a wall or obstruction, turn right
            // If the guard reaches a wall or obstruction, turn right
            if let Some(&cell) = grid.get(&next_pos) {
                if cell == '#' {
                    guard_dir = turn_right[&guard_dir]; // Turn right if there's an obstacle
                } else {
                    guard_pos = next_pos; // Move to the next position
                }
            } else {
                // Check if the next position is within bounds of the grid
                let (next_row, next_col) = next_pos;
                let max_row = grid.keys().map(|(r, _)| *r).max().unwrap_or(0);
                let max_col = grid.keys().map(|(_, c)| *c).max().unwrap_or(0);

                // If the next position is out of bounds, stop the guard
                if next_row <= max_row && next_col <= max_col {
                    guard_pos = next_pos; // Move to the next position if it's within bounds
                } else {
                    // Handle out-of-bounds (optional: stop the guard, wrap around, etc.)
                    // Here we just stop the guard, but you can decide on your desired behavior
                    return false; // Exit the loop if the guard moves out of bounds
                }
            }
        }
    }

    // let mut loop_positions = HashSet::new();
    let mut loop_positions: HashSet<(usize, usize)> = HashSet::new();

    // Step 1: Collect positions to test
    let mut test_positions = Vec::new();
    for (&pos, &cell) in grid.iter() {
        if cell == '.' {
            // Only test empty spaces
            test_positions.push(pos); // Collect positions that are empty
        }
    }

    // Step 2: Temporarily place obstructions and check for loops
    for pos in test_positions {
        // Temporarily place an obstruction
        grid.insert(pos, '#');

        // Simulate and check if a loop occurs
        if causes_loop(&grid, guard_pos, guard_dir) {
            loop_positions.insert(pos);
        }

        // Remove the obstruction after the test
        grid.insert(pos, '.');
    }

    Ok(loop_positions.len() as i32)
}
