use std::collections::HashMap;

fn calc(initial: &[i64], turns: i64) -> i64 {
    let mut last_spoken = HashMap::new();
    let mut turn = 1;
    let mut last = initial[0];
    for v in initial.iter().skip(1) {
        last_spoken.insert(last, turn);
        turn += 1;
        last = *v;
    }

    while turn < turns {
        let next_spoken = match last_spoken.get(&last) {
            Some(last_turn) => {
                turn - last_turn
            },
            None => {
                0
            }
        };

        last_spoken.insert(last, turn);
        turn += 1;
        last = next_spoken;
    }
    last
}

fn main() {
    let initial: Vec<i64> = vec![12,20,0,6,1,17,7];

    let part1 = calc(&initial, 2020);
    let part2 = calc(&initial, 30000000);

    println!( "{} {}", part1, part2 );
}
