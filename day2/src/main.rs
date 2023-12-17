use anyhow::Result;
use regex::Regex;
use std::str;
use std::{collections::HashMap, fs::File, io::Read};

fn main() -> Result<()> {
    let hm_limet: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let colors = ["red", "green", "blue"];
    let mut file = File::open("input.text").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let re = Regex::new(r"(\d+ [a-z])\w+").unwrap();
    let liens = str::from_utf8(&buffer)?.lines();
    let mut games_result: Vec<u32> = Vec::new();
    let mut nums_p2: Vec<u32> = Vec::new();

    for (game_id, game) in liens.enumerate() {
        let rounds = game.split(";");
        let mut hm_count = HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        for round in rounds {
            re.find_iter(round).map(|m| m.as_str()).for_each(|x| {
                let mut st_num: Vec<String> = vec![];
                let mut color: &str = "";
                for (i, st) in x.chars().enumerate() {
                    if st.is_numeric() {
                        st_num.push(st.to_string());
                    } else if st.is_whitespace() {
                    } else {
                        color = x.split_at(i).1;
                        break;
                    }
                }
                let num = st_num.join("").parse().unwrap_or(0);
                if hm_count.get(color) < Some(&num) {
                    hm_count.insert(color, num);
                }
            });
        }

        // part1 ----->>
        games_result.push(game_id as u32);
        for color in colors {
            if hm_limet.get(color) < hm_count.get(color) {
                games_result.pop();
                break;
            }
        }

        // part 2 ----->>
        let mut sum: u32 = 1;
        for color in colors {
            match hm_count.get(color) {
                Some(num) => sum = num * sum,
                _ => {}
            }
        }
        nums_p2.push(sum);
    }

    let result_p1: u32 = games_result.iter().fold(0, |state, num| state + num + 1);
    let result_p2: u32 = nums_p2.iter().fold(0, |state, num| state + num);

    println!("part1: {:?}", &result_p1);
    println!("part2: {:?}", result_p2);
    Ok(())
}
