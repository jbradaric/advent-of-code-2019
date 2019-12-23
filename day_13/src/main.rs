use std::{
    rc::Rc,
    cell::{Cell, RefCell},
    collections::HashMap,
};
use intcode::Intcode;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

impl From<i64> for Tile {
    fn from(v: i64) -> Tile {
        use Tile::*;
        [Empty, Wall, Block, Paddle, Ball][v as usize]
    }
}

fn part1(code: Vec<i64>) -> usize {
    let mut code = code;
    let mut count = 0;
    let mut out_count = 0;
    let mut x = 0;
    let mut y = 0;
    let screen = Rc::new(RefCell::new(HashMap::new()));
    code.run(&[], &mut |out| {
        out_count += 1;
        if out_count % 3 == 1 {
            x = out;
        } else if out_count % 3 == 2 {
            y = out;
        } else {
            let tile = Tile::from(out);
            if tile == Tile::Block {
                count += 1;
            }
            screen.borrow_mut().insert((x, y), tile);
        } 
    }).unwrap();
    display(&*screen.borrow(), 0);
    count
}

fn display(screen: &HashMap<(i64, i64), Tile>, score: i64) {
    let x_max = screen.keys().map(|(x, _)| x).max().unwrap();
    let y_max = screen.keys().map(|(_, y)| y).max().unwrap();

    print!("\x1b[2J\x1b[H");
    println!("Score: {}", score);
    for y in 0..*y_max {
        for x in 0..*x_max {
            let tile = screen.get(&(x, y)).unwrap_or(&Tile::Empty);
            let ch = match tile {
                Tile::Empty => ' ',
                Tile::Wall => '|',
                Tile::Block => '#',
                Tile::Paddle => '_',
                Tile::Ball => 'o',
            };
            print!("{}", ch);
        }
        print!("\n");
    }
    print!("\n");
}

fn part2(code: Vec<i64>) -> i64 {
    let mut code = code;
    let screen = Rc::new(RefCell::new(HashMap::new()));
    let score = Rc::new(Cell::new(0));

    fn find_tile_pos(screen: &HashMap<(i64, i64), Tile>, tile: Tile) -> (i64, i64) {
        for entry in screen.iter() {
            if entry.1 == &tile {
                return *entry.0;
            }
        }
        unreachable!();
    }

    code[0] = 2;

    let mut input = {
        let screen = Rc::clone(&screen);
        let score = Rc::clone(&score);
        move || {
            let ball_pos = find_tile_pos(&*screen.borrow(), Tile::Ball);
            let paddle_pos = find_tile_pos(&*screen.borrow(), Tile::Paddle);
            if ball_pos.0 < paddle_pos.0 {
                -1
            } else if ball_pos.0 > paddle_pos.0 {
                1
            } else {
                0
            }
        }
    };

    let mut output = {
        let screen = Rc::clone(&screen);
        let score = Rc::clone(&score);
        let mut out_count = 0;
        let mut x = 0;
        let mut y = 0;
        move |out| {
            out_count += 1;
            if out_count % 3 == 1 {
                x = out;
            } else if out_count % 3 == 2 {
                y = out;
            } else {
                if x == -1 && y == 0 {
                    score.set(out);
                } else {
                    let tile = Tile::from(out);
                    screen.borrow_mut().insert((x, y), tile);
                }
                // display(&*screen.borrow(), score.get());
                // std::thread::sleep(std::time::Duration::from_millis(1));
            } 
        }
    };
    code.run_iter(&mut input, &mut output).unwrap();

    score.get()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code: Vec<i64> = include_str!("input")
        .trim()
        .split(',')
        .map(|s| s.parse())
        .map(Result::unwrap)
        .collect();

    println!("Number of block tiles: {}", part1(code.clone()));
    println!("Final scope: {}", part2(code.clone()));

    Ok(())
}
