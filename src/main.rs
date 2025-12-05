mod _01;
mod _02;
mod _03;
mod _04;
mod _05;

use std::{env, process::exit};

fn usage() {
    println!("Usage:\n\t{{day}} | all")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Not enough args");
        usage();
        exit(-1);
    }

    if args[1] == "all" {
        _ = (1..=5).map(|x| dispatch(x)).collect::<Vec<_>>();
        exit(0);
    }

    let day = args[1].parse::<i32>().expect("Failed to parse day");
    dispatch(day);
    exit(0);
}

fn dispatch(day: i32) {
    println!(" Day {:02}\n--------", day);
    match day {
        1 => _01::run(),
        2 => _02::run(),
        3 => _03::run(),
        4 => _04::run(),
        5 => _05::run(),
        _ => {
            eprintln!("unknown day")
        }
    };
    println!();
}
