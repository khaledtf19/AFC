use anyhow::Result;
use regex::Regex;
use std::borrow::Cow;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{str, u32, u8};
use tokio::fs;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = PathBuf::from("input.text");
    part1(&file_path).await?;
    part2(&file_path).await?;

    Ok(())
}

async fn part2(file_path: &PathBuf) -> Result<()> {
    let mut fp = fs::File::open(file_path).await?;
    let mut buffer = Vec::new();
    fp.read_to_end(&mut buffer).await?;
    let possible_numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum: u32 = 0;

    let lines = str::from_utf8(&buffer)?.lines();
    for line in lines.clone() {
        let mut nums: Vec<String> = vec![];
        for (x, chr) in line.chars().enumerate() {
            if chr.is_numeric() {
                nums.push(chr.to_string());
            }
            let (_s, l) = line.split_at(x);
            for (i, num) in possible_numbers.iter().enumerate() {
                if l.starts_with(num) {
                    nums.push((i + 1).to_string())
                }
            }
        }
        let str = nums.get(0).unwrap_or(&"0".to_string()).clone()
            + nums.last().unwrap_or(&"0".to_string()).as_str();
        sum += str.parse().unwrap_or(0);
    }

    println!("part2: {}", sum);

    Ok(())
}

async fn part1(file_path: &PathBuf) -> Result<()> {
    let mut fp = fs::File::open(file_path).await?;

    let mut buffer = Vec::new();
    fp.read_to_end(&mut buffer).await?;

    let re = Regex::new(r"\d+").unwrap();

    let result: u32 = str::from_utf8(&buffer)?
        .split("\n")
        .map(|str| {
            re.find_iter(str)
                .map(|m| m.as_str())
                .collect::<Vec<&str>>()
                .join("")
        })
        .filter(|x| !x.is_empty())
        .map(|x| {
            return x.chars().nth(0).unwrap_or('0').to_string()
                + &x.chars().last().unwrap_or('0').to_string();
        })
        .fold(0, |state, x| x.parse::<u32>().unwrap() + state);
    println!("part1: {result}");

    Ok(())
}
