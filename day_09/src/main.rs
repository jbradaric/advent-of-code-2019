use intcode::Intcode;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = Vec::new();
    let mut code: Vec<i64> = {
        let mut file = File::open(env::args().nth(1).unwrap())?;
        let mut tmp = String::new();
        file.read_to_string(&mut tmp)?;
        tmp.trim()
            .split(',')
            .map(|s| s.parse())
            .map(Result::unwrap)
            .collect()
    };
    code.clone().run(&[1], &mut |r| output.push(r))?;
    println!("Part 1: {}", output[0]);

    output.clear();
    code.run(&[2], &mut |r| output.push(r))?;
    println!("Part 2: {}", output[0]);
    Ok(())
}
