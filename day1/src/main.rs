use anyhow::Result;
use regex::Regex;
use std::{str, u32};
use tokio::fs;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> Result<()> {
    let mut fp = fs::File::open("input.text").await?;
    let mut buffer = Vec::new();
    fp.read_to_end(&mut buffer).await?;
    let re = Regex::new(r"\d+").unwrap();

    let file: u32 = str::from_utf8(&buffer)?
        .split("\n")
        .map(|str| {
            re.find_iter(str)
                .map(|m| m.as_str())
                .collect::<Vec<&str>>()
                .join("")
        })
        .filter(|x| !x.is_empty())
        .map(|x| {
            return x.chars().nth(0).unwrap_or('s').to_string()
                + &x.chars().last().unwrap_or('s').to_string();
        })
        .fold(0, |state, x| x.parse::<u32>().unwrap() + state);

    println!("{:?}", file);

    Ok(())
}
