use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::u32;

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut result: Vec<u32> = Vec::new();
    for line_maybe in reader.lines() {
        match line_maybe {
            Ok(line) => part1(&mut result, line),
            Err(_) => {}
        }
    }

    let sum: u32 = result.iter().sum();
    println!("{}", sum);
    Ok(())
}
fn part1(result: &mut Vec<u32>, line: String) {
    let mut start = false;
    let mut is_second = false;
    let mut arr_nums: Vec<u32> = Vec::new();
    let mut num_string = String::from("");
    let mut result_num: u32 = 0;

    for (i, ch) in line.chars().enumerate() {
        if ch == ':' {
            start = true;
            continue;
        }
        if ch == '|' {
            is_second = true;
            continue;
        }
        if !start {
            continue;
        }
        if ch.is_ascii_digit() {
            num_string += ch.to_string().as_str();
            if i == line.len() - 1 {
                if arr_nums.contains(&num_string.parse().unwrap()) {
                    if result_num == 0 {
                        result_num = 1;
                    } else {
                        result_num *= 2
                    }
                }
                num_string = String::from("");
            }
            continue;
        }
        if ch.is_whitespace() && !&num_string.is_empty() {
            if !is_second {
                arr_nums.push(num_string.parse().unwrap());
            } else {
                if arr_nums.contains(&num_string.parse().unwrap_or(0)) {
                    if result_num == 0 {
                        result_num = 1;
                    } else {
                        result_num *= 2
                    }
                }
            }
            num_string = String::from("");
        }
    }
    result.push(result_num);
}
