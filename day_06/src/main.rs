use std::str::FromStr;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Error {
    InvalidOrbit(String)
}

struct Orbit {
    pub center: String,
    pub object: String
}

impl FromStr for Orbit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(')').collect();
        if parts.len() != 2 {
            Err(Error::InvalidOrbit(s.to_string()))
        } else {
            Ok(Orbit {
                center: parts[0].to_string(),
                object: parts[1].to_string()
            })
        }
    }
}

fn count_orbits_1(obj: &str, data: &HashMap<&str, &str>) -> u64 {
    let v = data.get(obj).unwrap();
    if v == &"COM" {
        1
    } else {
        1 + count_orbits_1(v, data)
    }
}

fn count_orbits(orbits: &Vec<Orbit>) -> u64 {
    let mut map: HashMap<&str, &str> = HashMap::new();
    for o in orbits.iter() {
        map.insert(&o.object, &o.center);
    }
    orbits.iter().map(|o| count_orbits_1(&o.object, &map)).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(env::args().nth(1).unwrap())?;
    let orbits: Vec<Orbit> = BufReader::new(file).lines()
        .map(Result::unwrap)
        .map(|s| s.parse())
        .map(Result::unwrap)
        .collect();
    dbg!(count_orbits(&orbits));
    Ok(())
}
