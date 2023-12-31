use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
};

mod d17;
use d17::{p1, p2};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number = args
        .get(1)
        .unwrap_or(&String::from("1"))
        .parse::<u32>()
        .unwrap_or(1);

    let input_file = match args.get(2) {
        Some(arg) => {
            if arg == "sample" {
                format!("inputs/d{}_sample.txt", day_number)
            } else {
                format!("inputs/d{}.txt", day_number)
            }
        }
        None => {
            format!("inputs/d{}.txt", day_number)
        }
    };

    let file = File::open(input_file).expect("no such file");
    let buf = BufReader::new(file);

    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    println!("{:?}", p1(&lines));
    println!("{:?}", p2(&lines));
}
