use std::env;
use std::fs::File;
use std::io::Read;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn output_image(pixels: &[u32]) {
    for row in pixels.chunks(WIDTH) {
        for pixel in row.iter() {
            if pixel == &0 {
                print!(" ");
            } else if pixel == &1 {
                print!("O");
            } else if pixel == &2 {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open(env::args().nth(1).expect("Specify input file"))?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;

    let digits: Vec<_> = s.trim().chars().map(|c| c.to_digit(10).unwrap()).collect();

    let checksum = digits.chunks(WIDTH * HEIGHT)
        .min_by_key(|&c| {
            c.iter().filter(|x| **x == 0).count()
        }).map(|c| {
            let ones = c.iter().filter(|x| **x == 1).count();
            let twos = c.iter().filter(|x| **x == 2).count();
            ones * twos
        }).unwrap();
    println!("Checksum: {}", checksum);

    let layers: Vec<_> = digits.chunks(WIDTH * HEIGHT)
        .map(|c| c.to_vec())
        .collect();
    let pixels: Vec<_> = (0..WIDTH * HEIGHT).map(|idx| {
        layers.iter()
            .map(|l| l[idx])
            .filter(|x| x != &2)
            .next().unwrap_or(2)
    }).collect();
    output_image(&pixels);

    Ok(())
}
