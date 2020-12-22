use advent::*;
#[derive(Debug)]
struct Bus {
    offset: usize,
    modulo: usize,
}

impl Bus {
    fn fits(&self, timestamp: usize) -> bool {
        (timestamp + self.offset) % self.modulo == 0
    }
}

fn main() -> Result<(), Error> {
    let timestamp = 1001612;
    let input_str = "19,x,x,x,x,x,x,x,x,41,x,x,x,37,x,x,x,x,x,821,x,x,x,x,x,x,x,x,x,x,x,x,13,x,x,x,17,x,x,x,x,x,x,x,x,x,x,x,29,x,463,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";
    let input = input_str
        .split(',')
        .filter_map(|x| {
            if x == "x" {
                None
            } else {
                Some(x.parse::<i64>().unwrap())
            }
        })
        .collect::<Vec<_>>();

    let mut wait_times = input
        .iter()
        .map(|bus_id| {
            let wait_time = bus_id - (timestamp % *bus_id);
            (wait_time, *bus_id)
        })
        .collect::<Vec<_>>();

    wait_times.sort_by_key(|(wait_time, _)| *wait_time);

    println!("{:?}", wait_times);

    println!("{}", wait_times[0].0 * wait_times[0].1);

    let mut buses = input_str
        .split(',')
        .enumerate()
        .filter_map(|(offset, s)| {
            if s == "x" {
                None
            } else {
                let id = s.parse::<usize>().unwrap();
                Some(Bus { offset, modulo: id })
            }
        })
        .collect::<Vec<_>>();

    println!("{:?}", buses);

    let mut ts = 0;
    let mut step = 1;
    for bus in buses {
        while !bus.fits(ts) {
            ts += step;
        }
        step *= bus.modulo;
    }

    println!("{}", ts);

    Ok(())
}
