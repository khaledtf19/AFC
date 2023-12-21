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
    let ins = instances.get_instances(y + 1);

    let split: Vec<&str> = line.split(":").collect();
    let nums: Vec<Vec<&str>> = split[1]
        .split("|")
        .map(|nums| {
            nums.split(" ")
                .map(|num| num.trim())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .collect();
    let result_nums: Vec<u32> = nums[0]
        .clone()
        .into_iter()
        .filter(|num| nums[1].contains(num))
        .map(|num| num.parse().unwrap())
        .collect();

    for i in y + 1..result_nums.len() as i32 + y + 1 {
        instances.join(i + 1, ins);
    }

    result.push(result_nums.iter().fold(0, |state, _| {
        if state == 0 {
            return 1;
        } else {
            return state * 2;
        }
    }) as u32);
}
