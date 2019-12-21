use std::env;
use std::fs::File;
use std::io::Read;
use intcode::Intcode;

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
    let input = [env::args().nth(2).unwrap().parse()?];
    code.as_mut_slice().run(&input, &mut |r| output.push(r))?;
    dbg!(output);
    Ok(())
}
