use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Map {
    name: String,
    right: String,
    left: String,
}

enum DirectionEnum {
    L,
    R,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file).lines();
    let dirs = reader
        .nth(0)
        .unwrap()
        .unwrap()
        .split("")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();
    let mut maps: Vec<Map> = Vec::new();

    for maybe_line in reader.skip(1) {
        match maybe_line {
            Ok(line) => resolve_line(line, &mut maps),
            Err(_) => {}
        }
    }
    let mut steps = 0;
    let mut curr_map_idx = 1;
    let mut curr_dir_idx = 0;
    let mut curr_search = "AAA".to_string();
    while curr_search != "ZZZ" {
        if curr_dir_idx >= dirs.len() {
            curr_dir_idx = 0;
        }
        if let Some(map) = maps.get(curr_map_idx) {
            if curr_search == map.name && dirs[curr_dir_idx] == "L" {
                curr_search = map.left.to_string();
                curr_map_idx += 1;
                curr_dir_idx += 1;
                steps += 1;
            } else if curr_search == map.name && dirs[curr_dir_idx] == "R" {
                curr_search = map.right.to_string();
                curr_map_idx += 1;
                curr_dir_idx += 1;
                steps += 1;
            } else {
                curr_map_idx += 1;
            }
        } else {
            curr_map_idx = 0;
        }
    }
    println!("{steps}");
    Ok(())
}
fn resolve_line(line: String, maps: &mut Vec<Map>) {
    let split = line.split(" = ").collect::<Vec<&str>>();
    let l_r = split[1]
        .split(", ")
        .map(|s| {
            s.chars()
                .filter(|ch| ch.is_ascii_alphabetic())
                .map(|ch| ch.to_string())
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<_>>();
    let map = Map {
        name: split[0].to_string(),
        right: l_r[1].to_owned(),
        left: l_r[0].to_owned(),
    };
    maps.push(map);
}
