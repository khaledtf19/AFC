use anyhow::Result;
use std::{
    fs::File,
    io::Read,
    str::{self},
    u32,
};

#[derive(Debug, Clone, Copy)]
struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone)]
struct Gear {
    nums_str: Vec<u32>,
    gear_point: Point,
}
impl Gear {
    pub fn add(&mut self, num: u32) {
        self.nums_str.push(num);
    }
    pub fn get_ratio(self) -> u32 {
        if self.nums_str.len() == 2 {
            return self.nums_str.iter().fold(1, |state, num| state * num);
        }
        0
    }
}

fn main() -> Result<()> {
    let mut file = File::open("input.text").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let map: Vec<Vec<char>> = str::from_utf8(&buffer)?
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    part1_part2(map);

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

fn part1_part2(map: Vec<Vec<char>>) {
    //  part1 --->
    let mut result_nums: Vec<u32> = Vec::new();

    //  part2 --->
    let mut gears: Vec<Gear> = Vec::new();

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
                    let res = search(&map, &pointes);
                    if !res.0 {
                        pointes = Vec::from([]);
                        nums_str = Vec::from([]);
                    } else {
                        //  part2 --->
                        if let Some(gears_points) = res.1 {
                            gears_points.iter().for_each(|new_gear| {
                                let gear = gears.iter().find(|g| {
                                    if g.gear_point.y == new_gear.y && g.gear_point.x == new_gear.x
                                    {
                                        return true;
                                    }
                                    false
                                });
                                if gear.is_none() {
                                    gears.push(Gear {
                                        nums_str: Vec::from([nums_str
                                            .join("")
                                            .clone()
                                            .parse()
                                            .unwrap()]),
                                        gear_point: Point {
                                            x: new_gear.x,
                                            y: new_gear.y,
                                        },
                                    })
                                } else {
                                    for g in gears.iter_mut() {
                                        if new_gear.x == g.gear_point.x
                                            && new_gear.y == g.gear_point.y
                                        {
                                            g.add(nums_str.join("").clone().parse().unwrap())
                                        }
                                    }
                                }
                            })
                        }
                        // part 1 --->
                        result_nums.push(nums_str.join("").clone().parse().unwrap());

                        // part 1,2 --->
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

    // part 1 --->
    let result: i64 = result_nums
        .iter()
        .fold(0 as i64, |state, num| *num as i64 + state as i64);
    println!("Part1: {result:?}");

    // part 2 --->
    println!(
        "Part2: {:?}",
        gears
            .into_iter()
            .fold(0, |state, gear| state + gear.get_ratio())
    );
}

fn search(map: &Vec<Vec<char>>, pointes: &Vec<Point>) -> (bool, Option<Vec<Point>>) {
    for curr in pointes.iter() {
        let res = walk(map, curr);
        if res.0 {
            return res;
        }
    }
    return (false, None);
}

fn walk(map: &Vec<Vec<char>>, curr: &Point) -> (bool, Option<Vec<Point>>) {
    let mut issymbol = false;
    let mut gear_points: Vec<Point> = Vec::new();
    for dir in DIR.iter() {
        let x = curr.x as i32 + dir.1;
        let y = curr.y as i32 + dir.0;
        if x < 0 || y < 0 {
            continue;
        }
        match map.get(y as usize) {
            Some(line) => {
                if x >= line.len() as i32 || y >= map.len() as i32 {
                    continue;
                }
                match line.get(x as usize) {
                    Some(ch) => {
                        if ch.is_ascii_punctuation() && ch != &'.' {
                            if ch == &'*' {
                                gear_points.push(Point {
                                    x: x as u32,
                                    y: y as u32,
                                });
                            }
                            issymbol = true;
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
    if gear_points.is_empty() {
        return (issymbol, None);
    }
    return (issymbol, Some(gear_points));
}
