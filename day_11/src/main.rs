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

fn print_registration(colors: &HashMap<(i64, i64), i64>) {
    let x1 = *colors.keys().map(|(x, _)| x).min().unwrap();
    let x2 = *colors.keys().map(|(x, _)| x).max().unwrap();
    let y1 = *colors.keys().map(|(_, y)| y).min().unwrap();
    let y2 = *colors.keys().map(|(_, y)| y).max().unwrap();
    for x in x1..=x2 {
        for y in y1..=y2 {
            let color = colors.get(&(x, y)).unwrap_or(&0);
            if color == &1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
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

    let painted = paint(Program::new(&code), 1)?;
    print_registration(&painted);

    Ok(())
}
