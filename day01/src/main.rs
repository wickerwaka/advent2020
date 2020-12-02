use advent::*;

fn main() -> Result<(), Error> {
    let values : Vec<i32> = read_list("day01/input.txt")?;

    for i1 in 0..values.len() {
        for i2 in i1 + 1..values.len() {
            let sum = values[i1] + values[i2];
            if sum == 2020 {
                println!("{} {} {}", values[i1], values[i2], values[i1] * values[i2]);
            }
        }
    }

    for i1 in 0..values.len() {
        for i2 in i1 + 1..values.len() {
            for i3 in i2 + 1..values.len() {
                let sum = values[i1] + values[i2] + values[i3];
                if sum == 2020 {
                    println!(
                        "{} {} {} {}",
                        values[i1],
                        values[i2],
                        values[i3],
                        values[i1] * values[i2] * values[i3]
                    );
                }
            }
        }
    }

    Ok(())
}
