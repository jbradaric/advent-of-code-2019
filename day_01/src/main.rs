use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn required_fuel(m: i64) -> i64 {
    let mut total = 0;
    let mut remaining = m;
    loop {
        remaining = remaining / 3 - 2;
        if remaining <= 0 {
            break;
        }
        total += remaining;
    }
    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(env::args().nth(1).unwrap())?;
    let reader = BufReader::new(file);
    let res: i64 = reader.lines()
        .map(Result::unwrap)
        .map(|s| s.parse())
        .map(Result::unwrap)
        .map(required_fuel)
        .sum();
    println!("Result: {}", res);
    Ok(())
}
