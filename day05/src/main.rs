use advent::*;

enum Partition {
    Left,
    Right,
    Front,
    Back,
}
struct Partitions(Vec<Partition>);

impl AdventParse for Partitions {
    fn parse(line: &str) -> Result<Partitions, Error> {
        let p: Result<Vec<_>, _> = line
            .chars()
            .map(|c| match c {
                'L' => Ok(Partition::Left),
                'R' => Ok(Partition::Right),
                'F' => Ok(Partition::Front),
                'B' => Ok(Partition::Back),
                _ => Err(anyhow!("{} is not valid", c)),
            })
            .collect();
        Ok(Partitions(p?))
    }
}

#[derive(Debug)]
struct SeatRange {
    row: (u64, u64),
    col: (u64, u64),
}
impl SeatRange {
    fn new() -> Self {
        Self {
            row: (0, 128),
            col: (0, 8),
        }
    }

    fn divide(self, p: &Partition) -> Self {
        let rmid = (self.row.0 + self.row.1) / 2;
        let cmid = (self.col.0 + self.col.1) / 2;
        match p {
            Partition::Left => Self {
                row: self.row,
                col: (self.col.0, cmid),
            },
            Partition::Right => Self {
                row: self.row,
                col: (cmid, self.col.1),
            },
            Partition::Front => Self {
                row: (self.row.0, rmid),
                col: self.col,
            },
            Partition::Back => Self {
                row: (rmid, self.row.1),
                col: self.col,
            },
        }
    }
}

fn main() -> Result<(), Error> {
    let boarding: Vec<Partitions> = read_list("day05/input.txt")?;

    let mut seats = Vec::new();
    for ticket in boarding.iter() {
        let f = ticket
            .0
            .iter()
            .fold(SeatRange::new(), |range, p| range.divide(p));
        assert_eq!(f.row.1, f.row.0 + 1);
        assert_eq!(f.col.1, f.col.0 + 1);

        let seatid = (f.row.0 * 8) + f.col.0;
        seats.push(seatid);
    }
    println!("Max ID {}", seats.iter().max().unwrap());

    seats.sort();
    for sl in seats.windows(2) {
        if (sl[0] + 2) == sl[1] {
            println!("Missing seat ID {}", sl[0] + 1);
        }
    }

    Ok(())
}
