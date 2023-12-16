use anyhow::Result;
use regex::Regex;
use std::str;
use std::{collections::HashMap, fs::File, io::Read};

fn main() -> Result<()> {
    let hm_limet = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let colors = ["red", "green", "blue"];
    let mut file = File::open("input.text").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let re = Regex::new(r"(\d+ [a-z])\w+").unwrap();
    let liens = str::from_utf8(&buffer)?.lines();
    let mut result_games: Vec<u32> = Vec::new();
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
        result_games.push(game_id as u32);
        for color in colors {
            if hm_limet.get(color) < hm_count.get(color) {
                result_games.pop();
                break;
            }
        }
    }
    let result: u32 = result_games.iter().fold(0, |state, num| state + num + 1);

    println!("{:?}", &result);
    Ok(())
}
