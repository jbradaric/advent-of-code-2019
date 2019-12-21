use std::collections::HashMap;
use intcode::Program;

use std::env;
use std::fs::File;
use std::io::Read;

enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn move_one(&self, current_pos: &(i64, i64)) -> (i64, i64) {
        use Direction::*;

        match *self {
            Up => (current_pos.0, current_pos.1 + 1),
            Down => (current_pos.0, current_pos.1 - 1),
            Left => (current_pos.0 - 1, current_pos.1),
            Right => (current_pos.0 + 1, current_pos.1)
        }
    }

    pub fn rotate(&self, code: i64) -> Self {
        use Direction::*;

        match code {
            0 => match *self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up
            },
            1 => match *self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up
            },
            _ => panic!("unknown rotate command")
        }
    }
}

use std::error::Error;

fn paint(mut program: Program, start_color: i64) -> Result<HashMap<(i64, i64), i64>, Box<dyn Error>> {
    let mut position = (0, 0);
    let mut direction = Direction::Up;
    let mut painted: HashMap<(i64, i64), i64> = HashMap::new();
    let mut color = start_color;
    loop {
        if let Some(color) = program.run_partial(&mut || color)? {
            painted.insert(position, color);
        } else {
            break;
        }
        if let Some(rotate_command) = program.run_partial(&mut || color)? {
            direction = direction.rotate(rotate_command);
            position = direction.move_one(&position);
        } else {
            break;
        }
        color = *painted.get(&position).unwrap_or(&0);
    }
    Ok(painted)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code: Vec<i64> = {
        let mut file = File::open(env::args().nth(1).unwrap())?;
        let mut tmp = String::new();
        file.read_to_string(&mut tmp)?;
        tmp.trim()
            .split(',')
            .map(|s| s.parse())
            .map(Result::unwrap)
            .collect()
    };
    println!("Num painted: {}", paint(Program::new(&code), 0)?.len());

    Ok(())
}
