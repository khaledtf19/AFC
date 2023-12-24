use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut times: Vec<String> = Vec::new();
    let mut dists: Vec<String> = Vec::new();
    let mut p1_result: Vec<_> = Vec::new();
    let mut p2_result: u32 = 0;

    for line_maybe in reader.lines() {
        match line_maybe {
            Ok(line) => resolve_line(line, &mut times, &mut dists),
            Err(_) => {}
        }
    }
    //  part 1 --->
    for (i, num) in times.iter().enumerate() {
        let (time, dis) = (
            num.parse::<u32>().unwrap(),
            dists[i].parse::<u32>().unwrap(),
        );
        let mut count = 0;
        for speed in 1..time {
            let d = speed * (time - speed);
            if d > dis {
                count += 1
            }
        }
        p1_result.push(count)
    }
    println!(
        "part1: {:?}",
        p1_result.iter().fold(1, |state, num| state * num)
    );

    //  part2 --->
    let (time, dist): (u64, u64) = (
        times.join("").parse().unwrap(),
        dists.join("").parse().unwrap(),
    );
    for speed in 1..time {
        let d = speed * (time - speed);
        if d > dist {
            p2_result += 1
        }
    }
    println!("part2: {p2_result}");
    Ok(())
}
fn resolve_line(line: String, times: &mut Vec<String>, dist: &mut Vec<String>) {
    let split = line.split(": ").collect::<Vec<&str>>();
    let nums = split[1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    if split[0] == "Time" {
        *times = nums
    } else {
        *dist = nums
    }
}
