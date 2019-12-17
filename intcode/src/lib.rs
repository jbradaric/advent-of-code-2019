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
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
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
            5 => Ok(Operator::JumpIfTrue),
            6 => Ok(Operator::JumpIfFalse),
            7 => Ok(Operator::LessThan),
            8 => Ok(Operator::Equals),
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
        use Operator::*;

        match self.op {
            Add | Mul | LessThan | Equals => 4,
            In | Out => 2,
            JumpIfTrue | JumpIfFalse => 3,
            Break => 1
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

#[derive(Debug)]
pub struct Program {
    pos: usize,
    done: bool,
    code: Vec<i64>
}

impl Program {
    pub fn new(code: &[i64]) -> Program {
        Program {
            pos: 0,
            code: code.to_vec(),
            done: false
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    fn get_code(&self) -> &[i64] {
        &self.code
    }

    pub fn run_partial<'a>(&mut self, input_iter: &mut impl FnMut() -> i64) -> Result<Option<i64>, Error> {
        use Operator as Op;
        use ParamMode as Mode;

        loop {
            let inst = Instruction::try_from(self.code[self.pos])?;
            match inst.op {
                Op::Add | Op::Mul => {
                    let op1 = match inst.m1 {
                        Mode::Position => self.code[self.code[self.pos + 1] as usize],
                        Mode::Immediate => self.code[self.pos + 1]
                    };
                    let op2 = match inst.m2 {
                        Mode::Position => self.code[self.code[self.pos + 2] as usize],
                        Mode::Immediate => self.code[self.pos + 2]
                    };
                    let dest = match inst.m3 {
                        Mode::Position => self.code[self.pos + 3] as usize,
                        Mode::Immediate => self.pos + 3 as usize
                    };
                    match inst.op {
                        Op::Add => self.code[dest] = op1 + op2,
                        Op::Mul => self.code[dest] = op1 * op2,
                        _ => panic!("How did this happen?")
                    };
                }
                Op::In => {
                    let dest = match inst.m1 {
                        Mode::Position => self.code[self.pos + 1] as usize,
                        Mode::Immediate => self.pos + 1
                    };
                    self.code[dest] = input_iter();
                }
                Op::Out => {
                    let dest = match inst.m1 {
                        Mode::Position => self.code[self.pos + 1] as usize,
                        Mode::Immediate => self.pos + 1
                    };
                    self.pos += inst.increment();
                    return Ok(Some(self.code[dest]));
                }
                Op::JumpIfTrue => {
                    let op1 = match inst.m1 {
                        Mode::Position => self.code[self.code[self.pos + 1] as usize],
                        Mode::Immediate => self.code[self.pos + 1]
                    };
                    let op2 = match inst.m2 {
                        Mode::Position => self.code[self.code[self.pos + 2] as usize],
                        Mode::Immediate => self.code[self.pos + 2]
                    };
                    if op1 != 0 {
                        self.pos = op2 as usize;
                        continue;
                    }
                }
                Op::JumpIfFalse => {
                    let op1 = match inst.m1 {
                        Mode::Position => self.code[self.code[self.pos + 1] as usize],
                        Mode::Immediate => self.code[self.pos + 1]
                    };
                    let op2 = match inst.m2 {
                        Mode::Position => self.code[self.code[self.pos + 2] as usize],
                        Mode::Immediate => self.code[self.pos + 2]
                    };
                    if op1 == 0 {
                        self.pos = op2 as usize;
                        continue;
                    }
                }
                Op::LessThan => {
                    let op1 = match inst.m1 {
                        Mode::Position => self.code[self.code[self.pos + 1] as usize],
                        Mode::Immediate => self.code[self.pos + 1]
                    };
                    let op2 = match inst.m2 {
                        Mode::Position => self.code[self.code[self.pos + 2] as usize],
                        Mode::Immediate => self.code[self.pos + 2]
                    };
                    let dest = match inst.m3 {
                        Mode::Position => self.code[self.pos + 3] as usize,
                        Mode::Immediate => self.pos + 3 as usize
                    };
                    self.code[dest] = if op1 < op2 { 1 } else { 0 }
                }
                Op::Equals => {
                    let op1 = match inst.m1 {
                        Mode::Position => self.code[self.code[self.pos + 1] as usize],
                        Mode::Immediate => self.code[self.pos + 1]
                    };
                    let op2 = match inst.m2 {
                        Mode::Position => self.code[self.code[self.pos + 2] as usize],
                        Mode::Immediate => self.code[self.pos + 2]
                    };
                    let dest = match inst.m3 {
                        Mode::Position => self.code[self.pos + 3] as usize,
                        Mode::Immediate => self.pos + 3 as usize
                    };
                    self.code[dest] = if op1 == op2 { 1 } else { 0 }
                }
                Op::Break => {
                    break;
                }
            };
            self.pos += inst.increment();
        }
        self.done = true;
        Ok(None)
    }
}

pub trait Intcode {
    fn run(&mut self, input: &[i64], output: &mut Vec<i64>) -> Result<(), Error>;
}

impl Intcode for &mut [i64] {
    fn run(&mut self, input: &[i64], output: &mut Vec<i64>) -> Result<(), Error> {
        let mut input_iter = input.iter();
        let mut func = move || {
            *input_iter.next().unwrap()
        };
        let mut prog = Program::new(self);
        while !prog.is_done() {
            if let Some(res) = prog.run_partial(&mut func)? {
                output.push(res);
            }
        }
        self[..].clone_from_slice(prog.get_code());
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

    #[test]
    fn test_eq_8_pos() {
        let mut code = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let mut input = [8];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [1]);
        output.clear();
        input[0] = 7;
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [0]);
    }

    #[test]
    fn test_lt_8_pos() {
        let mut code = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let mut input = [7];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [1]);
        code = vec![3,9,7,9,10,9,4,9,99,-1,8];
        output.clear();
        input[0] = 8;
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [0]);
    }

    #[test]
    fn test_eq_8_immediate() {
        let mut code = vec![3,3,1108,-1,8,3,4,3,99];
        let mut input = [8];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [1]);
        code = vec![3,3,1108,-1,8,3,4,3,99];
        output.clear();
        input[0] = 7;
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [0]);
    }

    #[test]
    fn test_lt_8_immediate() {
        let mut code = vec![3,3,1107,-1,8,3,4,3,99];
        let mut input = [7];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [1]);
        code = vec![3,3,1107,-1,8,3,4,3,99];
        output.clear();
        input[0] = 8;
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [0]);
    }

    #[test]
    fn test_jump_pos() {
        let mut code = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        let mut input = [0];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [0]);
        code = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        output.clear();
        input[0] = 1;
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [1]);
    }

    #[test]
    fn test_jump_imm() {
        let mut code = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        let mut input = [0];
        let mut output = Vec::new();
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [0]);

        code = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        output.clear();
        input[0] = 1;
        code.as_mut_slice().run(&input, &mut output).unwrap();
        assert_eq!(output, [1]);
    }
}
