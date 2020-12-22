use advent::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Cmd {
    Mask(u64, u64),
    Store(u64, u64),
}

impl AdventParse for Cmd {
    fn parse(s: &str) -> Result<Self, Error> {
        if let Some(mask_str) = s.strip_prefix("mask = ") {
            let mut mask = 0u64;
            let mut value = 0u64;
            for c in mask_str.chars() {
                match c {
                    '1' => {
                        mask |= 0;
                        value |= 1;
                    }
                    '0' => {
                        mask |= 0;
                        value |= 0;
                    }
                    'X' => {
                        mask |= 1;
                        value |= 0;
                    }
                    _ => panic!("uh"),
                };
                mask <<= 1;
                value <<= 1;
            }
            mask >>= 1;
            value >>= 1;
            Ok(Cmd::Mask(mask, value))
        } else if let Some(mem_str) = s.strip_prefix("mem") {
            let parts = mem_str
                .split(&['[', ']', ' ', '='][..])
                .filter(|x| x.len() > 0)
                .collect::<Vec<_>>();
            assert_eq!(parts.len(), 2);

            let address = parts[0].parse::<u64>()?;
            let value = parts[1].parse::<u64>()?;
            Ok(Cmd::Store(address, value))
        } else {
            Err(anyhow!("uh"))
        }
    }
}

fn store(mem: &mut HashMap<u64, u64>, addr: u64, floating: u64, value: u64) {
    let floating_positions = {
        let mut v = Vec::new();
        for i in 0..64 {
            if (floating & (1 << i)) != 0 {
                v.push(i);
            }
        }
        v
    };

    for p in 0..2i32.pow(floating_positions.len() as u32) {
        let mut float_addr = addr & (!floating);
        floating_positions
            .iter()
            .enumerate()
            .filter(|(idx, _)| ((p >> idx) & 1) == 1)
            .for_each(|(_, bit)| {
                float_addr |= 1 << bit;
            });
        mem.insert(float_addr, value);
    }
}

fn main() -> Result<(), Error> {
    let cmds: Vec<Cmd> = read_list("day14/input.txt")?;
    let max_addr = cmds
        .iter()
        .map(|x| match x {
            Cmd::Store(addr, _) => *addr,
            _ => 0,
        })
        .max()
        .unwrap();

    let mut memory = vec![0u64; max_addr as usize + 1];

    let mut mask = 0;
    let mut value = 0;
    for cmd in cmds.iter() {
        match cmd {
            Cmd::Mask(m, v) => {
                mask = *m;
                value = *v;
            }
            Cmd::Store(addr, v) => {
                let masked = *v & mask;
                memory[*addr as usize] = masked | value;
            }
        }
    }

    println!("{}", memory.iter().sum::<u64>());

    let mut memory2 = HashMap::new();
    let mut mask = 0;
    let mut value = 0;
    for cmd in cmds.iter() {
        match cmd {
            Cmd::Mask(m, v) => {
                mask = *m;
                value = *v;
            }
            Cmd::Store(addr, v) => {
                store(&mut memory2, addr | value, mask, *v);
            }
        }
    }

    println!("{}", memory2.values().sum::<u64>());

    Ok(())
}
