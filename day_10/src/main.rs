use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
struct Point(f64, f64);

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
        self.1.to_string().hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

#[derive(PartialOrd, Clone, Copy, Debug)]
struct Angle(f64);

impl Hash for Angle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Angle {}


fn unique_slopes(center: &Point, asteroids: &Vec<&&Point>) -> usize {
    let mut slopes = HashSet::new();
    for a in asteroids.iter() {
        let x = center.0 - a.0;
        let y = center.1 - a.1;
        slopes.insert(y.atan2(x).to_string());
    }
    slopes.len()
}

fn calc_slopes(center: &Point, asteroids: &Vec<&Point>) -> usize {
    let (x1, y1) = (center.0, center.1);
    let quad1: Vec<_> = asteroids
        .iter()
        .filter(|&p| p.0 <= x1 && p.1 <= y1)
        .collect();
    let quad2: Vec<_> = asteroids
        .iter()
        .filter(|&p| p.0 > x1 && p.1 <= y1)
        .collect();
    let quad3: Vec<_> = asteroids
        .iter()
        .filter(|&p| p.0 <= x1 && p.1 > y1)
        .collect();
    let quad4: Vec<_> = asteroids.iter().filter(|&p| p.0 > x1 && p.1 > y1).collect();
    [quad1, quad2, quad3, quad4]
        .iter()
        .map(|q| unique_slopes(center, q))
        .sum()
}

fn angle(start: &Point, end: &Point) -> Angle {
    use std::f64::consts::PI;
    let result = (end.0 - start.0).atan2(start.1 - end.1) * 180. / PI;
    Angle(if result < 0. {
        result + 360.
    } else {
        result
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = env::args().nth(1).expect("Input file not specified");
    let file = File::open(input_file)?;
    let coords: Vec<Vec<Point>> = BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.unwrap()
                .trim()
                .chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == '#' {
                        Some(Point(x as f64, y as f64))
                    } else {
                        None
                    }
                })
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect()
        })
        .collect();
    let coords2: Vec<_> = coords.iter().flatten().collect();
    let best = coords2
        .iter()
        .map(|c| (c, calc_slopes(c, &coords2)))
        .max_by_key(|(_, count)| *count)
        .unwrap();
    println!("Part 1: ({}, {}) = {}", (best.0).0, (best.0).1, best.1);

    let station = *best.0;
    let mut tmp: Vec<_> = coords2
        .iter()
        .filter(|&p| p != &station)
        .map(|p| (angle(&station, p), **p))
        .collect();
    tmp.sort_by(|&(a1, p1), &(a2, p2)| {
        a1.partial_cmp(&a2).unwrap()
            .then_with(|| {
                let d1 = (station.0 - p1.0).abs() + (station.1 - p1.1).abs();
                let d2 = (station.0 - p2.0).abs() + (station.1 - p2.1).abs();
                d1.partial_cmp(&d2).unwrap()
        })
    });
    let dead = kill(&tmp, 200);
    println!("Part 2: {}", dead.0 * 100. + dead.1);

    Ok(())
}

fn kill(asteroids: &Vec<(Angle, Point)>, num: usize) -> Point {
    let mut asteroids = asteroids.clone();
    let mut num_dead = 0;
    loop {
        let mut seen = HashSet::new();
        let mut rest = Vec::new();
        for (a, p) in asteroids.iter() {
            if seen.contains(a) {
                rest.push((*a, *p));
            } else {
                num_dead += 1;
                seen.insert(*a);
                if num_dead == num {
                    return p.clone();
                }
            }
        }
        asteroids.clear();
        asteroids.extend(rest);
        if asteroids.is_empty() {
            break;
        }
    }
    unreachable!();
}
