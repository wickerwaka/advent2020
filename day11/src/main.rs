use advent::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

struct Row(Vec<Cell>);

impl AdventParse for Row {
    fn parse(s: &str) -> Result<Self, Error> {
        let v: Result<Vec<Cell>, Error> = s
            .chars()
            .map(|c| match c {
                'L' => Ok(Cell::Empty),
                '.' => Ok(Cell::Floor),
                _ => Err(anyhow!("Unknown character: {}", c)),
            })
            .collect();
        Ok(Row(v?))
    }
}

#[derive(PartialEq, Clone)]
struct State {
    width: i64,
    height: i64,
    cells: Vec<Cell>,
}

impl State {
    fn from_rows(rows: &[Row]) -> State {
        let height = rows.len();
        let width = rows[0].0.len();
        let mut cells = Vec::with_capacity(height * width);
        for row in rows {
            cells.extend_from_slice(&row.0);
        }

        State {
            width: width as i64,
            height: height as i64,
            cells,
        }
    }

    fn get_cell(&self, x: i64, y: i64) -> Cell {
        if x < 0 || x >= self.width {
            Cell::Empty
        } else if y < 0 || y >= self.height {
            Cell::Empty
        } else {
            let idx = x + (y * self.width);
            self.cells[idx as usize]
        }
    }

    fn count_adjacent(&self, x: i64, y: i64) -> usize {
        let positions = vec![
            (x - 1, y - 1),
            (x + 0, y - 1),
            (x + 1, y - 1),
            (x - 1, y + 0),
            (x + 1, y + 0),
            (x - 1, y + 1),
            (x + 0, y + 1),
            (x + 1, y + 1),
        ];

        positions
            .iter()
            .filter(|(x, y)| matches!(self.get_cell(*x, *y), Cell::Occupied))
            .count()
    }

    fn count_visible(&self, x: i64, y: i64) -> usize {
        let directions: Vec<(i64, i64)> = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let mut visible = 0;

        for (dx, dy) in directions.into_iter() {
            let mut xx = x + dx;
            let mut yy = y + dy;

            loop {
                let cell = self.get_cell(xx, yy);
                if cell == Cell::Occupied {
                    visible += 1;
                    break;
                } else if cell == Cell::Empty {
                    break;
                }
                xx += dx;
                yy += dy;
            }
        }
        visible
    }

    fn update(&self) -> State {
        let mut cells = Vec::with_capacity(self.cells.len());
        for y in 0..self.height {
            for x in 0..self.width {
                let new_cell = match self.get_cell(x, y) {
                    Cell::Empty => {
                        if self.count_adjacent(x, y) == 0 {
                            Cell::Occupied
                        } else {
                            Cell::Empty
                        }
                    }
                    Cell::Occupied => {
                        if self.count_adjacent(x, y) >= 4 {
                            Cell::Empty
                        } else {
                            Cell::Occupied
                        }
                    }
                    x => x,
                };
                cells.push(new_cell);
            }
        }
        State {
            width: self.width,
            height: self.height,
            cells,
        }
    }

    fn update2(&self) -> State {
        let mut cells = Vec::with_capacity(self.cells.len());
        for y in 0..self.height {
            for x in 0..self.width {
                let new_cell = match self.get_cell(x, y) {
                    Cell::Empty => {
                        if self.count_visible(x, y) == 0 {
                            Cell::Occupied
                        } else {
                            Cell::Empty
                        }
                    }
                    Cell::Occupied => {
                        if self.count_visible(x, y) >= 5 {
                            Cell::Empty
                        } else {
                            Cell::Occupied
                        }
                    }
                    x => x,
                };
                cells.push(new_cell);
            }
        }
        State {
            width: self.width,
            height: self.height,
            cells,
        }
    }
}

fn main() -> Result<(), Error> {
    let rows: Vec<Row> = read_list("day11/input.txt")?;
    let initial_state = State::from_rows(&rows);

    let mut state = initial_state.clone();
    loop {
        let new_state = state.update();
        if new_state == state {
            break;
        }
        state = new_state;
    }

    let occupied_count = state.cells.iter().filter(|x| *x == &Cell::Occupied).count();
    println!("Occupied: {}", occupied_count);

    let mut state = initial_state.clone();
    loop {
        let new_state = state.update2();
        if new_state == state {
            break;
        }
        state = new_state;
    }

    let occupied_count = state.cells.iter().filter(|x| *x == &Cell::Occupied).count();
    println!("Occupied: {}", occupied_count);

    Ok(())
}
