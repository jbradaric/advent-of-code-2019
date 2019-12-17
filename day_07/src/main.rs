use intcode::Program;

use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Amplifier {
    program: Program,
}

impl Amplifier {
    pub fn new(code: &[i64]) -> Amplifier {
        Amplifier {
            program: Program::new(code),
        }
    }

    pub fn is_done(&self) -> bool {
        self.program.is_done()
    }

    pub fn run(&mut self, phase_setting: i64, input_signal: i64) -> i64 {
        let input = [phase_setting, input_signal];
        let mut input_iter = input.iter();
        let mut get_input = move || *input_iter.next().unwrap();
        self.program.run_partial(&mut get_input).unwrap().unwrap()
    }

    pub fn run_partial(&mut self, input: &[i64]) -> Option<i64> {
        let mut input_iter = input.iter();
        let mut get_input = move || *input_iter.next().unwrap();
        self.program.run_partial(&mut get_input).unwrap()
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

    let code: Vec<i64> = s
        .trim()
        .split(',')
        .map(|s| s.parse())
        .map(Result::unwrap)
        .collect();
    let max_signal = permutations(&[0, 1, 2, 3, 4])
        .iter()
        .map(|&p| {
            let mut amplifiers: Vec<_> = (0..5).map(|_| Amplifier::new(&code)).collect();
            let mut input = 0;
            for (amp, phase) in amplifiers.iter_mut().zip(p.iter()) {
                input = amp.run(*phase, input);
            }
            input
        })
        .max()
        .unwrap();
    println!("Max signal: {}", max_signal);

    let max_signal = permutations(&[5, 6, 7, 8, 9])
        .iter()
        .map(|&p| run_feedback_loop(&code, p))
        .max()
        .unwrap();
    println!("Max signal with feedback: {}", max_signal);

    Ok(())
}

fn run_feedback_loop(code: &[i64], p: [i64; 5]) -> i64 {
    let mut input = 0;
    let mut amps: Vec<_> = (0..5).map(|_| Amplifier::new(&code)).collect();
    let mut sent_phases = 0;
    let mut loop_done = false;
    loop {
        for (amp, phase) in amps.iter_mut().zip(p.iter()) {
            if amp.is_done() {
                loop_done = true;
                break;
            }
            if sent_phases < 5 {
                input = amp.run_partial(&[*phase, input]).unwrap();
                sent_phases += 1;
            } else {
                if let Some(out) = amp.run_partial(&[input]) {
                    input = out;
                }
            }
        }
        if loop_done {
            break;
        }
    }
    input
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let code = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
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
        let code = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
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
        let code = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let mut amplifiers: Vec<_> = (0..5).map(|_| Amplifier::new(&code)).collect();
        let phase_settings = [1i64, 0, 4, 3, 2];
        let mut input = 0;
        for (amp, phase) in amplifiers.iter_mut().zip(phase_settings.iter()) {
            input = amp.run(*phase, input);
        }
        assert_eq!(input, 65210);
    }

    #[test]
    fn test_feedback_1() {
        let code = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(run_feedback_loop(&code, [9, 8, 7, 6, 5]), 139629729);
    }
}
