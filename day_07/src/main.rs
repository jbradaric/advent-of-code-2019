use intcode::Intcode;

use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Amplifier {
    code: Vec<i64>,
}

impl Amplifier {
    pub fn new(code: &[i64]) -> Amplifier {
        Amplifier {
            code: code.to_vec()
        }
    }

    pub fn run(&self, phase_setting: i64, input_signal: i64) -> i64 {
        let mut code = self.code.clone();
        let mut output = Vec::new();
        code.as_mut_slice()
            .run(&[phase_setting, input_signal], &mut output)
            .expect("Run failed");
        output[output.len() - 1]
    }
}

fn permutations(state: &[i64; 5]) -> Vec<[i64; 5]> {
    let mut state = state.clone();
    let mut output = Vec::new();
    let mut c = [0; 5];

    output.push(state.clone());

    let mut i = 0;
    while i < 5 {
        if c[i] < i {
            if i % 2 == 0 {
                state.swap(0, i);
            } else {
                state.swap(c[i], i);
            }
            output.push(state.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    output
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(env::args().nth(1).unwrap())?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let code: Vec<i64> = s.trim().split(',')
        .map(|s| s.parse())
        .map(Result::unwrap)
        .collect();
    let max_signal = permutations(&[0, 1, 2, 3, 4]).iter()
        .map(|&p| {
            let mut amplifiers: Vec<_> = (0..5).map(|_| Amplifier::new(&code)).collect();
            let mut input = 0;
            for (amp, phase) in amplifiers.iter_mut().zip(p.iter()) {
                input = amp.run(*phase, input);
            }
            input
        }).max().unwrap();
    println!("Max signal: {}", max_signal);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let code = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let mut amplifiers: Vec<_> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let phase_settings = [4i64, 3, 2, 1, 0];
        let mut input = 0;
        for (amp, phase) in amplifiers.iter_mut().zip(phase_settings.iter()) {
            input = amp.run(*phase, input);
        }
        assert_eq!(input, 43210);
    }

    #[test]
    fn test_2() {
        let code = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
        let mut amplifiers: Vec<_> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let phase_settings = [0i64, 1, 2, 3, 4];
        let mut input = 0;
        for (amp, phase) in amplifiers.iter_mut().zip(phase_settings.iter()) {
            input = amp.run(*phase, input);
        }
        assert_eq!(input, 54321);
    }

    #[test]
    fn test_3() {
        let code = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let mut amplifiers: Vec<_> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let phase_settings = [1i64, 0, 4, 3, 2];
        let mut input = 0;
        for (amp, phase) in amplifiers.iter_mut().zip(phase_settings.iter()) {
            input = amp.run(*phase, input);
        }
        assert_eq!(input, 65210);
    }
}
