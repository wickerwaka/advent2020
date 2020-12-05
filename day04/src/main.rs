use advent::*;
use std::collections::HashSet;

fn main() -> Result<(), Error> {
    let input = std::fs::read_to_string("day04/input.txt")?;

    let required_keys: HashSet<&str> = vec![ "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid" ].into_iter().collect();

    let mut valid_keys = 0;
    let mut valid_values = 0;
    let mut total = 0;

    for blocks in input.split("\n\n") {
        let mut keys = HashSet::new();
        let mut values_valid = true;
        for kv in blocks.split_whitespace() {
            let mut it = kv.split(':');
            let key = it.next().unwrap();
            let value = it.next().unwrap();

            let v = match key {
                "byr" => {
                    let y = value.parse::<i32>().unwrap_or(0);
                    value.len() == 4 && y >= 1920 && y <= 2002
                },
                "iyr" => {
                    let y = value.parse::<i32>().unwrap_or(0);
                    value.len() == 4 && y >= 2010 && y <= 2020
                },
                "eyr" => {
                    let y = value.parse::<i32>().unwrap_or(0);
                    value.len() == 4 && y >= 2020 && y <= 2030
                },
                "hgt" => {
                    if let Some(v) = value.strip_suffix("cm") {
                        let y = v.parse::<i32>().unwrap_or(0);
                        y >= 150 && y <= 193
                    } else if let Some(v) = value.strip_suffix("in") {
                        let y = v.parse::<i32>().unwrap_or(0);
                        y >= 59 && y <= 76
                    } else {
                        false
                    }
                },
                "hcl" => {
                    if let Some(v) = value.strip_prefix("#") {
                        u64::from_str_radix(v, 16).is_ok()
                    } else {
                        false
                    }
                },
                "ecl" => {
                    match value {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                        _ => false
                    }
                },
                "pid" => {
                    value.len() == 9 && value.parse::<u64>().is_ok()
                },
                _ => false,
            };

            values_valid &= v;

            keys.insert(key);
        }

        total += 1;

        if keys.is_superset(&required_keys) {
            valid_keys += 1;
            if values_valid {
                valid_values += 1;
            }
        }
    }

    println!( "total: {} keys: {} keys_and_values: {}", total, valid_keys, valid_values );

    Ok(())
}
