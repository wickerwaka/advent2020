use advent::*;

fn is_valid(target: i64, history: &[i64]) -> bool {
    for x in 0..history.len() {
        for y in x..history.len() {
            if history[x] + history[y] == target {
                return true;
            }
        }
    }
    return false;
}

fn find_sum_slice(target: i64, numbers: &[i64]) -> Option<&[i64]> {
    for start in 0..numbers.len() {
        for end in start + 1..numbers.len() {
            let sl = &numbers[start..end];
            if sl.iter().sum::<i64>() == target {
                return Some(sl);
            }
        }
    }
    None
}

fn main() -> Result<(), Error> {
    let numbers: Vec<i64> = read_list("day09/input.txt")?;
    let history_len = 32;
    let mut invalid_number = None;
    for window in numbers.windows(history_len + 1) {
        let history = &window[..history_len];
        let target = window[history_len];

        if !is_valid(target, history) {
            println!("{}", target);
            invalid_number = Some(target);
            break;
        }
    }

    let sl = find_sum_slice(invalid_number.unwrap(), &numbers).unwrap();
    let mn = sl.iter().min().unwrap();
    let mx = sl.iter().max().unwrap();
    println!("{:?} {} {} {}", sl, mn, mx, mn + mx);

    Ok(())
}
