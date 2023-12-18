use anyhow::Result;
use std::{
    char,
    fs::File,
    i32,
    io::Read,
    str::{self},
    u32,
};

#[derive(Debug)]
struct Point {
    pub x: u32,
    pub y: u32,
}

fn main() -> Result<()> {
    let mut file = File::open("input.text").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let map: Vec<Vec<char>> = str::from_utf8(&buffer)?
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut result_nums: Vec<u32> = Vec::new();

    for (y, line) in map.iter().enumerate() {
        let mut nums_str: Vec<String> = Vec::new();
        let mut pointes: Vec<Point> = Vec::new();

        for (x, char) in line.iter().enumerate() {
            if !char.is_numeric() || x == line.len() - 1 {
                if char.is_numeric() {
                    pointes.push(Point {
                        x: x as u32,
                        y: y as u32,
                    });
                    nums_str.push(char.to_string());
                }
                if !pointes.is_empty() {
                    if !search(&map, &pointes) {
                        pointes = Vec::from([]);
                        nums_str = Vec::from([]);
                    } else {
                        result_nums.push(nums_str.join("").parse().unwrap());
                        pointes = Vec::from([]);
                        nums_str = Vec::from([]);
                    }
                }
            }
            if char.is_numeric() {
                pointes.push(Point {
                    x: x as u32,
                    y: y as u32,
                });
                nums_str.push(char.to_string());
            }
        }
    }

    let result: i64 = result_nums
        .iter()
        .fold(0 as i64, |state, num| *num as i64 + state as i64);
    println!("{result:?}");

    Ok(())
}

const DIR: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn search(map: &Vec<Vec<char>>, pointes: &Vec<Point>) -> bool {
    for curr in pointes.iter() {
        if walk(map, curr) {
            return true;
        }
    }
    false
}

fn walk(map: &Vec<Vec<char>>, curr: &Point) -> bool {
    for dir in DIR.iter() {
        let x = curr.x as i32 + dir.1;
        let y = curr.y as i32 + dir.0;
        if x < 0 || y < 0 {
            continue;
        }
        println!("y: {}, x: {}", y, x);
        match map.get(y as usize) {
            Some(line) => {
                if x >= line.len() as i32 || y >= map.len() as i32 {
                    continue;
                }
                match line.get(x as usize) {
                    Some(ch) => {
                        if ch.is_ascii_punctuation() && ch != &'.' {
                            return true;
                        }
                    }
                    None => {
                        continue;
                    }
                }
            }
            None => {
                continue;
            }
        }
    }
    false
}
