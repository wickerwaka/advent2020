use advent::*;

struct Password {
    letter: char,
    min: usize,
    max: usize,
    password: String
}

impl AdventParse for Password {
    fn parse(line: &str) -> Result<Password, Error> {
        let parts = line.split(&['-', ' ', ':'][..]).filter(|x| x.len() > 0).collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(anyhow!("Does not contain four parts"));
        }
        Ok(Password {
            min: parts[0].parse()?,
            max: parts[1].parse()?,
            letter: parts[2].chars().next().ok_or(anyhow!( "could not read character" ))?,
            password: parts[3].to_string()
        })
    }
}

fn main() -> Result<(), Error> {
    let passwords : Vec<Password> = read_list("day02/input.txt")?;

    let valid = passwords.iter().filter(|x| {
        let count = x.password.chars().filter(|c| *c == x.letter).count();
        count >= x.min && count <= x.max
    }).count();
    println!( "{}", valid );

    let valid = passwords.iter().filter(|x| {
        let first = x.password.chars().nth(x.min - 1).map(|c| c == x.letter).unwrap_or(false);
        let second = x.password.chars().nth(x.max - 1).map(|c| c == x.letter).unwrap_or(false);

        first != second
    }).count();

    println!("{}", valid);
    
    Ok(())
}
