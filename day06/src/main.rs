use advent::*;
use std::collections::HashMap;

#[derive(Debug)]
enum Input {
    Answers(String),
    Terminator
}

impl AdventParse for Input {
    fn parse(line: &str) -> Result<Self, Error> {
        if line.len() == 0 {
            Ok(Input::Terminator)
        } else {
            Ok(Input::Answers(line.to_string()))
        }
    }
}

fn main() -> Result<(), Error> {
    let input : Vec<Input> = read_list("day06/input.txt")?;
    let mut total_or = 0;
    let mut total_and = 0;
    for group in input.split(|x| matches!( x, Input::Terminator)) {
        let mut all_chars = HashMap::new();
        for a in group {
            match a {
                Input::Answers(ref s) => {
                    for c in s.chars() {
                        *all_chars.entry(c).or_insert(0usize) += 1;
                    }
                },
                _ => {}
            };
        }

        let sz = group.len();

        total_or += all_chars.len();
        total_and += all_chars.values().into_iter().filter(|v| **v == sz).count();
    }

    println!( "Sum: {} {}", total_or, total_and);

    Ok(())
}
