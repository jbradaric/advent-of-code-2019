use std::env;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;
use std::convert::From;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum MoveError {
    NumberError(ParseIntError),
    DirectionError
}

impl From<ParseIntError> for MoveError {
    fn from(e: ParseIntError) -> MoveError {
        MoveError::NumberError(e)
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Up(i64),
    Down(i64),
    Left(i64),
    Right(i64)
}

impl Move {
    fn dir(&self) -> (i64, i64) {
        use Move::*;

        match *self {
            Up(_) => (0, 1),
            Down(_) => (0, -1),
            Left(_) => (-1, 0),
            Right(_) => (1, 0)
        }
    }

    fn len(&self) -> i64 {
        use Move::*;

        match *self {
            Up(n) => n,
            Down(n) => n,
            Left(n) => n,
            Right(n) => n,
        }
    }

    pub fn get_points(&self, start: &Point) -> HashSet<Point> {
        let mut set = HashSet::new();
        let (dx, dy) = self.dir();
        let mut start = Point(start.0 + dx, start.1 + dy);
        for _ in 0..self.len() {
            set.insert(start.clone());
            start.0 += dx;
            start.1 += dy;
        }
        set
    }

    fn steps_between(&self, start: &Point, end: &Point) -> Option<u64> {
        let (dx, dy) = self.dir();
        let real_end = Point(start.0 + dx * self.len(), start.1 + dy * self.len());
        if end.0 != start.0 && end.1 != start.1 {
            return None;  // point is not on the line
        }
        if end.0 == start.0 {
            let b1 = start.1.min(real_end.1);
            let b2 = start.1.max(real_end.1);
            if b1 <= end.1 && end.1 <= b2 {
                Some((end.1 - start.1).abs() as u64)
            } else {
                None
            }
        } else if end.1 == start.1 {
            let b1 = start.0.min(real_end.0);
            let b2 = start.0.max(real_end.0);
            if b1 <= end.0 && end.0 <= b2 {
                Some((end.0 - start.0).abs() as u64)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl std::str::FromStr for Move {
    type Err = MoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Move::*;

        match &s[..1] {
            "U" => Ok(Up(s[1..].parse()?)),
            "D" => Ok(Down(s[1..].parse()?)),
            "L" => Ok(Left(s[1..].parse()?)),
            "R" => Ok(Right(s[1..].parse()?)),
            _ => Err(MoveError::DirectionError)
        }
    }
}

#[derive(Clone, Copy, Debug, Eq)]
struct Point(i64, i64);

impl Point {
    fn dist_to_orig(&self) -> u64 {
        self.0.abs() as u64 + self.1.abs() as u64
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist_to_orig().cmp(&other.dist_to_orig())
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
        self.1.hash(state);
    }
}

#[derive(Debug)]
struct Wire {
    pub moves: Vec<(Point, Move)>
}

impl From<String> for Wire {
    fn from(s: String) -> Wire {
        let mut start = Point(0, 0);
        let mut moves = vec![];
        for part in s.trim().split(',') {
            let dir: Move = part.parse().unwrap();
            moves.push((start, dir));
            let (dx, dy) = dir.dir();
            start = Point(start.0 + dx * dir.len(), start.1 + dy * dir.len());
        }
        Wire { moves: moves }
    }
}

fn intersection(p1: &(Point, Move), p2: &(Point, Move)) -> Option<Point> {
    let s1 = p1.1.get_points(&p1.0);
    let s2 = p2.1.get_points(&p2.0);
    s1.intersection(&s2).min().map(|p| *p)
}

fn wires_cross(w1: &Wire, w2: &Wire) -> Option<Point> {
    let mut all = HashSet::new();
    for pos1 in w1.moves.iter() {
        for pos2 in w2.moves.iter() {
            if let Some(p) = intersection(&pos1, &pos2) {
                all.insert(p);
            }
        }
    }
    all.iter()
        .min_by(|p, q| {
            let s1 = num_steps(w1, **p) + num_steps(w2, **p);
            let s2 = num_steps(w1, **q) + num_steps(w2, **q);
            s1.cmp(&s2)
        })
        .map(|p| *p)
}

fn num_steps(w: &Wire, intersection: Point) -> u64 {
    let mut count = 0;
    for (p, m) in w.moves.iter() {
        if let Some(c) = m.steps_between(p, &intersection) {
            count += c;
            break;
        } else {
            count += m.len() as u64;
        }
    }
    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(env::args().nth(1).unwrap())?;
    let wires: Vec<Wire> = BufReader::new(file).lines()
        .map(Result::unwrap)
        .map(Wire::from)
        .collect();
    for (pos, wire) in wires.iter().enumerate() {
        for other in wires[pos+1..].iter() {
            if let Some(p) = wires_cross(wire, other) {
                println!("Total: {}", num_steps(&wire, p) + num_steps(&other, p));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_steps() {
        let w1 = Wire::from("R8,U5,L5,D3".to_string());
        assert_eq!(num_steps(&w1, Point(3, 3)), 20);
    }

    #[test]
    fn test_1() {
        let w1 = Wire::from("R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string());
        let w2 = Wire::from("U62,R66,U55,R34,D71,R55,D58,R83".to_string());
        let x = wires_cross(&w1, &w2).unwrap();
        assert_eq!(num_steps(&w1, x) + num_steps(&w2, x), 610);
    }

    #[test]
    fn test_2() {
        let w1 = Wire::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".to_string());
        let w2 = Wire::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string());
        let x = wires_cross(&w1, &w2).unwrap();
        assert_eq!(num_steps(&w1, x) + num_steps(&w2, x), 410);
    }
}
