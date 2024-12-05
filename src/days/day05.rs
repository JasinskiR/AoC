use code_timing_macros::time_snippet;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const DAY: usize = 5;

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

fn parse_input(lines: &[String]) -> (HashMap<i32, HashMap<i32, i32>>, Vec<Vec<i32>>) {
    let mut hash_map: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    let mut vector: Vec<Vec<i32>> = Vec::new();

    let (pairs, lists) =
        lines.split_at(lines.iter().position(|line| !line.contains('|')).unwrap());

    for pair in pairs {
        if let Some((key, value)) = pair.split_once('|') {
            let key = key.parse::<i32>().unwrap();
            let value = value.parse::<i32>().unwrap();

            hash_map
                .entry(key)
                .or_insert_with(HashMap::new)
                .insert(value, 1);
        }
    }

    for list in lists {
        let parsed_list: Vec<i32> = list
            .split(',')
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        vector.push(parsed_list);
    }

    (hash_map, vector)
}

fn is_valid_page_sequence(pages: &[i32], page_order_map: &HashMap<i32, HashMap<i32, i32>>) -> bool {
    for (i, &current_page) in pages.iter().enumerate() {
        for &next_page in pages.iter().skip(i + 1) {
            if let Some(valid_next_pages) = page_order_map.get(&current_page) {
                if !valid_next_pages.contains_key(&next_page) {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn part1(lines: &[String]) -> Result<i32, io::Error> {
    let (page_order_map, printed_manual) = parse_input(lines);

    let mut middle_sum = 0;

    for pages in &printed_manual {
        if pages.is_empty() {
            continue;
        }

        if is_valid_page_sequence(pages, &page_order_map) {
            middle_sum += pages[pages.len() / 2];
        }
    }

    Ok(middle_sum)
}

fn part2(lines: &[String]) -> Result<i32, io::Error> {
    let (page_order_map, printed_manual) = parse_input(lines);

    let mut middle_sum = 0;
    let mut invalid_updates = Vec::new();

    for pages in &printed_manual {
        if pages.is_empty() {
            continue;
        }

        if !is_valid_page_sequence(pages, &page_order_map) {
            invalid_updates.push(pages.clone());
        }
    }

    for invalid_update in invalid_updates {
        let mut page_used_rule_counts: HashMap<i32, usize> = HashMap::new();

        for &current_page in &invalid_update {
            if let Some(valid_next_pages) = page_order_map.get(&current_page) {
                let used_rules_count = valid_next_pages.keys().filter(|&&next_page| invalid_update.contains(&next_page)).count();
                page_used_rule_counts.insert(current_page, used_rules_count);
            }
        }

        let mut sorted_pages = invalid_update;
        sorted_pages.sort_by_key(|page| std::cmp::Reverse(page_used_rule_counts.get(page).cloned().unwrap_or(0)));

        if !sorted_pages.is_empty() {
            middle_sum += sorted_pages[sorted_pages.len() / 2];
        }
    }

    Ok(middle_sum)
}
