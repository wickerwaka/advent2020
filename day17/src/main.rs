use advent::*;
use std::i64;
use std::ops::RangeInclusive;
use std::iter::Iterator;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Clone, Eq)]
struct Point(Vec<i64>);

/*
struct Neighbors {
    center: Point,
    iter: 
}

impl Iterator for Neighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|p|
        Point(
            self.center.0 + p.0,
            self.center.1 + p.1,
            self.center.2 + p.2,
        ))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
*/

impl Point {
    fn maximum(k: usize) -> Self {
        Point(vec![ i64::MAX; k ])
    }
    fn minimum(k: usize) -> Self {
        Point(vec![ i64::MIN; k ])
    }

    fn zero(k: usize) -> Self {
        Point(vec![ 0; k ])
    }

    fn min(&self, other: &Point) -> Point {
        assert_eq!( self.0.len(), other.0.len() );
        let v : Vec<i64> = self.0.iter().zip(other.0.iter()).map(|(a, b)| *a.min(b)).collect();
        Point(v)
    }

    fn max(&self, other: &Point) -> Point {
        assert_eq!( self.0.len(), other.0.len() );
        let v : Vec<i64> = self.0.iter().zip(other.0.iter()).map(|(a, b)| *a.max(b)).collect();
        Point(v)
    }

    fn add(&self, other: &Point) -> Point {
        assert_eq!( self.0.len(), other.0.len() );
        let v : Vec<i64> = self.0.iter().zip(other.0.iter()).map(|(a, b)| *a + *b).collect();
        Point(v)
    }

    fn neighbors(&self) -> impl Iterator<Item=Point> + '_ {
        let center = self.clone();
        let ranges = vec![ (-1..=1); self.0.len() ];
        ranges.into_iter().multi_cartesian_product().filter(|v| !v.iter().all(|x| *x == 0)).map(move |v| {
            center.add(&Point(v))
        })
    }
}

#[derive(Clone)]
struct Universe {
    dimensions: usize,
    active: HashSet<Point>,
}

impl Universe {
    fn new(points: &[Point]) -> Self {
        Self {
            dimensions: points[0].0.len(),
            active: points.iter().cloned().collect::<HashSet<_>>(),
        }
    }

    fn outer_bounds(
        &self,
    ) -> Vec<RangeInclusive<i64>>
    {
        let mut mn = Point::maximum(self.dimensions);
        let mut mx = Point::minimum(self.dimensions);

        for p in self.active.iter() {
            mn = p.min(&mn);
            mx = p.max(&mx);
        }

        mn.0.iter().zip(mx.0.iter()).map(|(mn, mx)| {
            RangeInclusive::new(mn - 1, mx + 1)
        }).collect()
    }

    fn update1(self) -> Self {
        let mut new_actives = HashSet::new();
        for p in self.outer_bounds().into_iter().multi_cartesian_product() {
            let center = Point(p);
            let neighbor_count = center.neighbors().filter(|p| {
                self.active.contains(p)
            }).count();

            if self.active.contains(&center) {
                if neighbor_count == 2 || neighbor_count == 3 {
                    new_actives.insert(center);
                }
            } else {
                if neighbor_count == 3 {
                    new_actives.insert(center);
                }
            }
        }

        Universe {
            dimensions: self.dimensions,
            active: new_actives
        }
    }
}

fn main() -> Result<(), Error> {
    let input = std::fs::read_to_string("day17/input.txt")?;

    let mut points3 = Vec::new();
    let mut points4 = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                let p = Point(vec![x as i64, y as i64, 0]);
                points3.push(p);
                let p = Point(vec![x as i64, y as i64, 0, 0]);
                points4.push(p);
            }
        }
    }

    let universe = Universe::new(&points3);

    let mut state = universe.clone();
    for _ in 0..6 {
        state = state.update1();
    }

    println!( "{}", state.active.len());

    let universe = Universe::new(&points4);

    let mut state = universe.clone();
    for _ in 0..6 {
        state = state.update1();
    }

    println!( "{}", state.active.len());





    
    Ok(())
}
