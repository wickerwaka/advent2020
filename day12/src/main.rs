use advent::*;

#[derive(Debug, Clone, Copy)]
enum Command {
    Forward(i64),
    Left(i64),
    Right(i64),
    East(i64),
    West(i64),
    North(i64),
    South(i64),
}

impl AdventParse for Command {
    fn parse(s: &str) -> Result<Self, Error> {
        let c = &s[0..1];
        let num = s[1..].parse::<i64>()?;

        match c {
            "F" => Ok(Command::Forward(num)),
            "L" => Ok(Command::Left(num)),
            "R" => Ok(Command::Right(num)),
            "E" => Ok(Command::East(num)),
            "W" => Ok(Command::West(num)),
            "S" => Ok(Command::South(num)),
            "N" => Ok(Command::North(num)),
            _ => Err(anyhow!("Unknown command: {}", s)),
        }
    }
}

struct Ship {
    direction: i64,
    x: i64,
    y: i64,
}

impl Ship {
    fn new() -> Self {
        Self {
            direction: 90,
            x: 0,
            y: 0,
        }
    }

    fn run(&mut self, command: Command) {
        match command {
            Command::East(steps) => self.x += steps,
            Command::West(steps) => self.x -= steps,
            Command::North(steps) => self.y -= steps,
            Command::South(steps) => self.y += steps,
            Command::Left(ang) => self.direction -= ang,
            Command::Right(ang) => self.direction += ang,
            Command::Forward(steps) => {
                if self.direction < 0 {
                    self.direction = 360 - ((-self.direction) % 360);
                } else {
                    self.direction = self.direction % 360;
                }
                match self.direction {
                    0 => self.y -= steps,
                    90 => self.x += steps,
                    180 => self.y += steps,
                    270 => self.x -= steps,
                    _ => panic!("uh"),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Ship2 {
    wx: i64,
    wy: i64,
    x: i64,
    y: i64,
}

impl Ship2 {
    fn new() -> Self {
        Self {
            wx: 10,
            wy: -1,
            x: 0,
            y: 0,
        }
    }

    fn run(&mut self, command: Command) {
        println!("{:?} ::: {:?}", command, self);
        match command {
            Command::East(steps) => self.wx += steps,
            Command::West(steps) => self.wx -= steps,
            Command::North(steps) => self.wy -= steps,
            Command::South(steps) => self.wy += steps,
            Command::Right(ang) => {
                let x = self.wx;
                let y = self.wy;
                match ang {
                    0 => {}
                    90 => {
                        self.wx = -y;
                        self.wy = x;
                    }
                    180 => {
                        self.wx = -x;
                        self.wy = -y;
                    }
                    270 => {
                        self.wx = y;
                        self.wy = -x;
                    }
                    _ => panic!("what"),
                }
            }
            Command::Left(ang) => {
                let x = self.wx;
                let y = self.wy;
                match ang {
                    0 => {}
                    90 => {
                        self.wx = y;
                        self.wy = -x;
                    }
                    180 => {
                        self.wx = -x;
                        self.wy = -y;
                    }
                    270 => {
                        self.wx = -y;
                        self.wy = x;
                    }
                    _ => panic!("what"),
                }
            }
            Command::Forward(steps) => {
                self.x += self.wx * steps;
                self.y += self.wy * steps;
            }
        }
        println!("---> {:?}", self);
    }
}

fn main() -> Result<(), Error> {
    let commands: Vec<Command> = read_list("day12/input.txt")?;

    let mut ship = Ship::new();

    for command in commands.iter() {
        ship.run(*command);
    }
    println!("{}", ship.x.abs() + ship.y.abs());

    let mut ship2 = Ship2::new();
    for command in commands.iter() {
        ship2.run(*command);
    }

    println!("{}", ship2.x.abs() + ship2.y.abs());

    Ok(())
}
