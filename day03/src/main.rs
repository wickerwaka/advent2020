use advent::*;

struct Row {
    trees: Vec<bool>
}

impl AdventParse for Row {
    fn parse(line: &str) -> Result<Self, Error> {
        Ok( Row { trees: line.chars().map(|c| c == '#').collect() })
    }
}

fn count_hits(rows: &[Row], delta_x: usize, delta_y: usize) -> usize {
    let mut hits = 0;
    for (row, x) in rows.iter().step_by(delta_y).zip((0..).step_by(delta_x)) {
        let local_x = x % row.trees.len();
        if row.trees[local_x] {
            hits += 1;
        }
    }
    hits
}

fn main() -> Result<(), Error> {
    let rows : Vec<Row> = read_list("day03/input.txt")?;
    
    let hits = count_hits(&rows, 3, 1);
    println!("Hits: {}", hits);

    let deltas = vec![ (1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut mul_hits = 1;
    for (delta_x, delta_y) in deltas.into_iter() {
        mul_hits *= count_hits(&rows, delta_x, delta_y);
    }
    println!( "Mul Hits: {}", mul_hits);

    Ok(())
}
