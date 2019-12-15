use std::convert::TryFrom;

#[derive(Debug)]
pub enum Error {
    UnknownOpcode(i64),
    UnknownMode(u32),
    UnknownModeChar(char),
    UnexpectedMode(ParamMode)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;

        match self {
            UnknownOpcode(c) => write!(f, "Unknown opcode: {}", c),
            UnknownMode(m) => write!(f, "Unknown mode: {}", m),
            UnknownModeChar(c) => write!(f, "Unknown mode: {}", c),
            UnexpectedMode(m) => write!(f, "Unexpected mode: {:?}", m)
        }
    }
}

impl std::error::Error for Error {}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Add,
    Mul,
    In,
    Out,
    Break
}

impl TryFrom<i64> for Operator {
    type Error = Error;

    fn try_from(code: i64) -> Result<Self, Self::Error> {
        match code {
            1 => Ok(Operator::Add),
            2 => Ok(Operator::Mul),
            3 => Ok(Operator::In),
            4 => Ok(Operator::Out),
            99 => Ok(Operator::Break),
            _ => Err(Error::UnknownOpcode(code))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ParamMode {
    Position,
    Immediate
}

impl TryFrom<u32> for ParamMode {
    type Error = Error;

    fn try_from(mode: u32) -> Result<Self, Self::Error> {
        match mode {
            0 => Ok(ParamMode::Position),
            1 => Ok(ParamMode::Immediate),
            _ => Err(Error::UnknownMode(mode))
        }
    }
}

impl TryFrom<char> for ParamMode {
    type Error = Error;

    fn try_from(mode: char) -> Result<Self, Self::Error> {
        match mode.to_digit(10) {
            Some(d) => ParamMode::try_from(d),
            None => Err(Error::UnknownModeChar(mode))
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub op: Operator,
    pub m1: ParamMode,
    pub m2: ParamMode,
    pub m3: ParamMode
}

impl Instruction {
    pub(crate) fn increment(&self) -> usize {
        match self.op {
            Operator::Add | Operator::Mul => 4,
            Operator::In | Operator::Out => 2,
            Operator::Break => 1
        }
    }
}

impl TryFrom<i64> for Instruction {
    type Error = Error;

    fn try_from(instruction: i64) -> Result<Self, Self::Error> {
        let op = Operator::try_from(instruction % 100)?;
        let v: Vec<_> = format!("{:05}", instruction).chars().collect();
        let mode1 = ParamMode::try_from(v[2])?;
        let mode2 = ParamMode::try_from(v[1])?;
        let mode3 = ParamMode::try_from(v[0])?;
        Ok(Instruction {
            op: op,
            m1: mode1,
            m2: mode2,
            m3: mode3
        })
    }
}

pub trait Intcode {
    fn run(&mut self, input: &[i64], output: &mut Vec<i64>) -> Result<(), Error>;
}

impl Intcode for &mut [i64] {
    fn run(&mut self, input: &[i64], output: &mut Vec<i64>) -> Result<(), Error> {
        use Operator as Op;
        use ParamMode as Mode;

        let mut input_iter = input.iter();
        let mut pos = 0;
        loop {
            let inst = Instruction::try_from(self[pos])?;
            match inst.op {
                Op::Add | Op::Mul => {
                    let op1 = match inst.m1 {
                        Mode::Position => self[self[pos + 1] as usize],
                        Mode::Immediate => self[pos + 1]
                    };
                    let op2 = match inst.m2 {
                        Mode::Position => self[self[pos + 2] as usize],
                        Mode::Immediate => self[pos + 2]
                    };
                    let dest = match inst.m3 {
                        Mode::Position => self[pos + 3] as usize,
                        Mode::Immediate => Err(Error::UnexpectedMode(inst.m3))?
                    };
                    match inst.op {
                        Op::Add => self[dest] = op1 + op2,
                        Op::Mul => self[dest] = op1 * op2,
                        _ => panic!("How did this happen?")
                    };
                }
                Op::In => {
                    let dest = match inst.m1 {
                        Mode::Position => self[pos + 1] as usize,
                        Mode::Immediate => pos + 1
                    };
                    self[dest] = *input_iter.next().unwrap();
                }
                Op::Out => {
                    let dest = match inst.m1 {
                        Mode::Position => self[pos + 1] as usize,
                        Mode::Immediate => pos + 1
                    };
                    output.push(self[dest]);
                }
                Op::Break => {
                    break;
                }
            };
            pos += inst.increment();
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let mut code = vec![1, 0, 0, 0, 99];
        let input = [];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(code, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_2() {
        let mut code = vec![2, 3, 0, 3, 99];
        let input = [];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(code, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_3() {
        let mut code = vec![2, 4, 4, 5, 99, 0];
        let input = [];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(code, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_4() {
        let mut code = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let input = [];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(code,   vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_5() {
        let mut code = vec![3, 0, 4, 0, 99];
        let input = [11];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [11]);
    }
}
