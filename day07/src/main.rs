use std::collections::HashMap;
use std::fs::read;

use advent::*;

#[derive(Debug)]
struct BagGroup {
    color: String,
    count: usize,
}
#[derive(Debug)]
struct BagType {
    color: String,
    groups: Vec<BagGroup>,
}

impl AdventParse for BagType {
    fn parse(line: &str) -> Result<Self, Error> {
        let words: Vec<_> = line.split(&['.', ',', ' '][..]).collect();
        let mut name = None;
        let mut bag_groups = Vec::new();
        for segment in words.split(|w| {
            *w == "bags" || *w == "bag" || *w == "contains" || *w == "contain" || *w == ""
        }) {
            if segment.len() == 0 {
                continue;
            }
            if name.is_none() {
                name.replace(segment.join(" "));
            } else {
                match segment {
                    &["no", "other"] => {}
                    &[x, y] => bag_groups.push(BagGroup {
                        color: format!("{} {}", x, y),
                        count: 1,
                    }),
                    &[s_count, x, y] => {
                        let count: usize = s_count.parse()?;
                        bag_groups.push(BagGroup {
                            color: format!("{} {}", x, y),
                            count,
                        });
                    }
                    _ => {}
                };
            }
        }

        Ok(BagType {
            color: name.unwrap(),
            groups: bag_groups,
        })
    }
}

fn contains(search: &str, start: &str, bag_map: &HashMap<&str, &BagType>) -> bool {
    if search == start {
        return true;
    }

    if let Some(bag_type) = bag_map.get(start) {
        for bg in bag_type.groups.iter() {
            if contains(search, &bg.color, bag_map) {
                return true;
            }
        }
    }
    return false;
}

fn aggregate(start: &str, bag_map: &HashMap<&str, &BagType>) -> usize {
    let mut sum = 0;
    if let Some(bag_type) = bag_map.get(start) {
        sum = 1;
        for bg in bag_type.groups.iter() {
            sum += bg.count * aggregate(&bg.color, bag_map);
        }
    }
    return sum;
}

fn main() -> Result<(), Error> {
    let bag_types: Vec<BagType> = read_list("day07/input.txt")?;

    let mut bag_map = HashMap::new();
    for bt in bag_types.iter() {
        bag_map.insert(bt.color.as_str(), bt);
    }

    let count = bag_types
        .iter()
        .filter(|x| contains("shiny gold", &x.color, &bag_map))
        .count();
    let agg = aggregate("shiny gold", &bag_map);
    println!("{} {}", count - 1, agg - 1);

    Ok(())
}
