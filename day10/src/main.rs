use std::fs::read;

use advent::*;

fn main() -> Result<(), Error> {
    let joltages = {
        let mut v: Vec<i64> = read_list("day10/input.txt")?;
        v.sort();
        v
    };

    let device_joltage = joltages.iter().max().map(|x| *x + 3).unwrap();
    let mut joltage: i64 = 0;

    let mut diffs = vec![0, 0, 0, 1];
    for j in joltages.iter() {
        let diff = j - joltage;
        diffs[diff as usize] += 1;
        joltage = *j;
    }
    println!("{:?}", diffs);
    println!("{}", diffs[1] * diffs[3]);

    let joltages = {
        let mut v = vec![0];
        v.extend_from_slice(&joltages);
        v.push(device_joltage);
        v
    };

    let mut slice_start = 0;
    let mut slices = Vec::new();
    for idx in 1..joltages.len() {
        let diff = joltages[idx] - joltages[idx - 1];
        if diff == 3 {
            let slice = &joltages[slice_start..idx];
            if slice.len() > 2 {
                slices.push(slice);
            }
            slice_start = idx;
        }
    }
    let slice = &joltages[slice_start..joltages.len()];
    if slice.len() > 2 {
        slices.push(slice);
    }

    let mut combinations = 1u64;
    for sl in slices {
        match sl.len() {
            3 => combinations *= 2,
            4 => combinations *= 4,
            5 => combinations *= 7,
            _ => panic!("uh"),
        }
    }

    println!("{}", combinations);

    Ok(())
}
