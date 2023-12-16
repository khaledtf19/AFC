use anyhow::Result;
use regex::Regex;
use std::path::PathBuf;
use std::{str, u32};
use tokio::fs;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let file_path = PathBuf::from("input.text");
    part1(&file_path).await?;
    part2(&PathBuf::from("input_text_p2.text")).await?;

    Ok(())
}

async fn part2(file_path: &PathBuf) -> Result<()> {
    let mut fp = fs::File::open(file_path).await?;
    let mut buffer = Vec::new();
    fp.read_to_end(&mut buffer).await?;

    let lines: Vec<_> = str::from_utf8(&buffer)?
        .lines()
        .map(|line| line.replace("two", "2"))
        .collect();
    println!("{lines:?}");

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
