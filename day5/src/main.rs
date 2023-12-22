use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut seeds: Vec<i64> = Vec::new();
    let mut maps: Vec<Vec<(i64, i64, i64)>> = Vec::new();
    let mut new_map: Vec<(i64, i64, i64)> = Vec::new();
    for line_maybe in reader.lines() {
        match line_maybe {
            Ok(line) => resolve_line(line, &mut maps, &mut new_map, &mut seeds),
            Err(_) => {}
        }
    }
    maps.push(new_map);
    println!("{maps:?}");
    let mut locations: Vec<i64> = Vec::new();
    for seed in seeds.into_iter() {
        let mut curr_num = seed;
        for map in maps.clone().into_iter() {
            for (dst_start, src_start, range) in map.into_iter() {
                if src_start <= curr_num && curr_num < src_start + range {
                    curr_num = dst_start + (curr_num - src_start);
                    break;
                }
            }
        }
        locations.push(curr_num);
    }
    println!("{:?}", locations);
    println!(
        "{:?}",
        locations.iter().fold(locations[0], |state, num| {
            if state > *num {
                return *num;
            }
            state
        })
    );
    Ok(())
}
fn resolve_line(
    line: String,
    maps: &mut Vec<Vec<(i64, i64, i64)>>,
    new_map: &mut Vec<(i64, i64, i64)>,
    seeds: &mut Vec<i64>,
) {
    if line.starts_with("seeds") {
        line.split(": ").collect::<Vec<&str>>()[1]
            .split(" ")
            .for_each(|num| seeds.push(num.parse().unwrap()));
    }
    if line.is_empty() && !new_map.is_empty() {
        maps.push(new_map.clone());
        new_map.clear();
    }

    if line
        .chars()
        .collect::<Vec<char>>()
        .first()
        .unwrap_or(&'1')
        .is_ascii_digit()
        && !line.is_empty()
    {
        let nums = line
            .split(" ")
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i64>>();
        let nums = (nums[0], nums[1], nums[2]);
        new_map.push(nums);
    }
}
