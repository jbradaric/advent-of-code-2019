use std::{
    env,
    cmp::Ordering,
    fs::File,
    str::FromStr,
    error::Error,
    num::ParseIntError,
    io::{BufRead, BufReader}
};

#[derive(Clone, Debug, Eq, Default)]
struct Vector {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl FromStr for Vector {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<_> = s.trim()
            .trim_start_matches(|c| c == '<')
            .trim_end_matches(|c| c == '>')
            .split(',')
            .map(|p| p.trim().split('=').skip(1).next().unwrap().parse().unwrap())
            .collect();
        Ok(Vector {
            x: coords[0],
            y: coords[1],
            z: coords[2]
        })
    }
}

#[derive(Clone, Debug)]
struct Moon {
    pub position: Vector,
    pub velocity: Vector
}

impl Moon {
    pub fn new(position: Vector) -> Moon {
        Moon {
            position: position,
            velocity: Default::default()
        }
    }

    pub fn update_velocity(&mut self, diff: Vector) {
        self.velocity.x += diff.x;
        self.velocity.y += diff.y;
        self.velocity.z += diff.z;
    }

    pub fn update_position(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn get_diff(&self, a: i64, b: i64) -> i64 {
        match a.cmp(&b) {
            Ordering::Less => 1,
            Ordering::Greater => -1,
            Ordering::Equal => 0
        }
    }
    
    pub fn velocity_diff(&self, other: &Moon) -> (Vector, Vector) {
        let x_diff = self.get_diff(self.position.x, other.position.x);
        let y_diff = self.get_diff(self.position.y, other.position.y);
        let z_diff = self.get_diff(self.position.z, other.position.z);

        let v1 = Vector { x: x_diff, y: y_diff, z: z_diff };
        let v2 = Vector { x: v1.x * -1, y: v1.y * -1, z: v1.z * -1 };

        (v1, v2)
    }
}

// updates velocity
fn apply_gravity(moons: &mut Vec<Moon>) {
    for idx1 in 0..moons.len() - 1 {
        for idx2 in idx1 + 1..moons.len() {
            if idx1 != idx2 {
                let (v1, v2) = moons[idx1].velocity_diff(&moons[idx2]);
                moons[idx1].update_velocity(v1);
                moons[idx2].update_velocity(v2);
            }
        }
    }
}

// updates position
fn apply_velocity(moons: &mut Vec<Moon>) {
    for m in moons.iter_mut() {
        m.update_position();
    }
}

fn simulate(moons: &mut Vec<Moon>) {
    apply_gravity(moons);
    apply_velocity(moons);
}

fn total_energy(moons: &Vec<Moon>) -> i64 {
    let mut total = 0;
    for m in moons.iter() {
        let pot = m.position.x.abs() + m.position.y.abs() + m.position.z.abs();
        let kin = m.velocity.x.abs() + m.velocity.y.abs() + m.velocity.z.abs();
        total += pot * kin;
    }
    total
}

macro_rules! steps_to {
    ($start:expr, $attr:ident) => {
        {
            let mut num_steps = 0;
            let mut moons = $start.clone();
            loop {
                simulate(&mut moons);
                num_steps += 1;
                if moons.iter().zip($start.iter())
                    .all(|(m1, m2)| {
                        m1.position.$attr == m2.position.$attr &&
                            m1.velocity.$attr == 0
                }) {
                    break;
                }
            }
            num_steps
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(env::args().nth(1).expect("Specify input filename"))?;
    let start: Vec<_> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| s.parse())
        .map(Result::unwrap)
        .map(Moon::new)
        .collect();

    let mut moons = start.clone();
    for _ in 0..1000 {
        simulate(&mut moons);
    }
    println!("Total energy: {}", total_energy(&moons));

    let x_steps = steps_to!(&start, x);
    let y_steps = steps_to!(&start, y);
    let z_steps = steps_to!(&start, z);
    println!("Total cycle steps: {}", lcm(lcm(x_steps, y_steps), z_steps));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cycle() {
        let start = vec![
            Moon::new(Vector { x: -1, y: 0, z: 2 }),
            Moon::new(Vector { x: 2, y: -10, z: -7 }),
            Moon::new(Vector { x: 4, y: -8, z: 8 }),
            Moon::new(Vector { x: 3, y: 5, z: -1 })
        ];
        let x_steps = steps_to!(&start, x);
        let y_steps = steps_to!(&start, y);
        let z_steps = steps_to!(&start, z);
        assert_eq!(lcm(lcm(x_steps, y_steps), z_steps), 2772);
    }
}
