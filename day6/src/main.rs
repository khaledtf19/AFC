use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut times: Vec<u32> = Vec::new();
    let mut dist: Vec<u32> = Vec::new();
    let mut result: Vec<_> = Vec::new();

    for line_maybe in reader.lines() {
        match line_maybe {
            Ok(line) => resolve_line(line, &mut times, &mut dist),
            Err(_) => {}
        }
    }
    for (i, num) in times.iter().enumerate() {
        let (time, dis) = (num, dist[i]);
        let mut count = 0;
        for speed in 1..*time {
            let d = speed * (time - speed);
            if d > dis {
                count += 1
            }
        }
        result.push(count)
    }
    println!("{:?}", result.iter().fold(1, |state, num| state * num));
    Ok(())
}
fn resolve_line(line: String, times: &mut Vec<u32>, dist: &mut Vec<u32>) {
    let split = line.split(": ").collect::<Vec<&str>>();
    let nums = split[1]
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();
    if split[0] == "Time" {
        *times = nums
    } else {
        *dist = nums
    }
}
