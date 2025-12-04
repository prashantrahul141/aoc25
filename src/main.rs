mod _01;
mod _02;
mod _03;
mod _04;

use std::{env, ops::Index, process::exit};

fn usage() {
    println!("Usage:\n\t{{day}}")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Not enough args");
        usage();
        exit(-1);
    }

    let day = args.index(1).parse::<i32>().expect("Failed to parse day");
    dispatch(day);
}

fn dispatch(day: i32) {
    match day {
        1 => _01::run(),
        2 => _02::run(),
        3 => _03::run(),
        4 => _04::run(),
        _ => {
            eprintln!("unknown day")
        }
    };
}
