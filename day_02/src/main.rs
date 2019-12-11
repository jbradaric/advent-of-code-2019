use std::env;
use std::fs::File;
use std::io::Read;

fn run_code(code: &mut [usize]) {
    let mut pos = 0;
    loop {
        match code[pos] {
            1 => { code[code[pos + 3]] = code[code[pos + 1]] + code[code[pos + 2]]; },
            2 => { code[code[pos + 3]] = code[code[pos + 1]] * code[code[pos + 2]]; },
            99 => { break; },
            _ => panic!("Unknown opcode"),
        };
        pos += 4;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut code: Vec<usize> = {
        let mut file = File::open(env::args().nth(1).unwrap())?;
        let mut tmp = String::new();
        file.read_to_string(&mut tmp)?;
        tmp.trim()
            .split(',')
            .map(|s| s.parse())
            .map(Result::unwrap)
            .collect()
    };

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut code = code.clone();
            code[1] = noun;
            code[2] = verb;
            run_code(&mut code);
            if code[0] == 19690720 {
                println!("noun = {}, verb = {}", noun, verb);
                return Ok(());
            }
        }
    }

    run_code(&mut code);
    println!("Result: {}", code[0]);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::run_code;

    #[test]
    fn test_1() {
        let mut code = vec![1, 0, 0, 0, 99];
        run_code(&mut code);
        assert_eq!(code, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_2() {
        let mut code = vec![2, 3, 0, 3, 99];
        run_code(&mut code);
        assert_eq!(code, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_3() {
        let mut code = vec![2, 4, 4, 5, 99, 0];
        run_code(&mut code);
        assert_eq!(code, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_4() {
        let mut code = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run_code(&mut code);
        assert_eq!(code,   vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
