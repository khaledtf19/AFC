use anyhow::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Instances {
    pub hm: HashMap<i32, i32>,
}
impl Instances {
    fn new() -> Instances {
        Instances { hm: HashMap::new() }
    }
    fn join(&mut self, key: i32, value: i32) {
        if let Some(num) = self.hm.get(&key) {
            self.hm.insert(key, num + value);
        } else {
            self.hm.insert(key, value + 1);
        }
    }
    fn get_instances(&mut self, key: i32) -> i32 {
        if let Some(instance) = self.hm.get(&key) {
            instance.clone()
        } else {
            self.hm.insert(key, 1);
            1
        }
    }
}

fn main() -> Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut instances = Instances::new();
    let mut result: Vec<u32> = Vec::new();
    for (y, line_maybe) in reader.lines().enumerate() {
        match line_maybe {
            Ok(line) => resolve_line(&mut result, line, &mut instances, y as i32),
            Err(_) => {}
        }
    }

    let sum: u32 = result.iter().sum();

    //  part 1 --->
    println!("part 1: {}", sum);
    //  part 2 --->
    println!("part 2: {:?}", instances.hm.values().sum::<i32>());
    Ok(())
}
fn resolve_line(result: &mut Vec<u32>, line: String, instances: &mut Instances, y: i32) {
    let mut start = false;
    let mut is_second = false;
    let mut arr_nums: Vec<u32> = Vec::new();
    let mut num_string = String::from("");
    let mut result_num: u32 = 0;
    let ins = instances.get_instances(y + 1);
    let mut count = 0;

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
                        count += 1;
                    } else {
                        result_num *= 2;
                        count += 1;
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
                        count += 1;
                    } else {
                        result_num *= 2;
                        count += 1;
                    }
                }
            }
            num_string = String::from("");
        }
    }

    for i in y + 1..count + y + 1 {
        instances.join(i + 1, ins);
    }

    result.push(result_num);
}
