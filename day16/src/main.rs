use advent::*;
use regex::Regex;
use std::ops::RangeInclusive;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct ValidParam {
    name: String,
    ranges: Vec<RangeInclusive<usize>>,
}

impl ValidParam {
    fn is_valid(&self, v: usize) -> bool {
        self.ranges.iter().any(|x| x.contains(&v))
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<usize>,
}

fn main() -> Result<(), Error> {
    let input = std::fs::read_to_string("day16/input.txt")?;

    let param_re = Regex::new(r"(?m)^(.+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();
    let valid_params = param_re
        .captures_iter(&input)
        .map(|caps| {
            let name = caps[1].to_string();
            let r1 = RangeInclusive::new(caps[2].parse().unwrap(), caps[3].parse().unwrap());
            let r2 = RangeInclusive::new(caps[4].parse().unwrap(), caps[5].parse().unwrap());
            ValidParam{
                name,
                ranges: vec![ r1, r2 ]
            }
        }).collect::<Vec<_>>();

    let ticket_re = Regex::new(r"(?m)^\d+,.*$").unwrap();
    let all_tickets = ticket_re.captures_iter(&input).map(|caps| {
        let values = caps[0].split(',').map(|x| x.parse().unwrap()).collect::<Vec<_>>();
        Ticket {
            fields: values
        }
    }).collect::<Vec<_>>();

    let my_ticket = &all_tickets[0];
    let other_tickets = &all_tickets[1..];

    let mut rate = 0;
    let mut valid_tickets = Vec::new();
    for ticket in other_tickets {
        let mut is_valid = true;
        for field in &ticket.fields {
            if !valid_params.iter().any(|x| x.is_valid(*field)) {
                rate += *field;
                is_valid = false;
            }
        }
        if is_valid {
            valid_tickets.push(ticket);
        }
    }

    valid_tickets.push(my_ticket);

    println!( "{}", rate );

    let mut possible_positions: HashMap<String, Vec<usize>> = HashMap::new();
    for param in &valid_params {
        let mut positions = HashMap::new();
        for ticket in &valid_tickets {
            for (pos, value) in ticket.fields.iter().enumerate() {
                if param.is_valid(*value) {
                    *positions.entry(pos).or_insert(0) += 1;
                }
            }
        }

        for (k, v) in positions.iter() {
            if *v == valid_tickets.len() {
                possible_positions.entry(param.name.clone()).or_default().push(*k);
            }
        }
    }

    let mut kvp = possible_positions.iter().collect::<Vec<_>>();
    kvp.sort_by_cached_key(|(_, v)| v.len());
    
    let mut assigned = HashSet::new();

    let mut product = 1;

    for (idx, hm) in kvp.iter().enumerate() {
        assert_eq!( idx + 1, hm.1.len());

        let s : HashSet<usize> = hm.1.iter().cloned().collect();
        let diff = s.difference(&assigned).collect::<Vec<_>>();
        assert_eq!(diff.len(), 1);
        let pos = *diff[0];
        assigned.insert(pos);

        if hm.0.starts_with("departure") {
            product *= my_ticket.fields[pos];
        }
    }

    println!( "{}", product );

    Ok(())
}
