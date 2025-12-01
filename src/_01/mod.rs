#[derive(Debug)]
enum Move {
    L(i32),
    R(i32),
}

impl From<(&'static str, &'static str)> for Move {
    fn from(value: (&'static str, &'static str)) -> Self {
        let mag = value.1.parse::<i32>().unwrap();
        match value.0 {
            "L" => Move::L(mag),
            "R" => Move::R(mag),
            _ => panic!("unknown direction"),
        }
    }
}

fn parse(i: &'static str) -> Vec<Move> {
    i.lines()
        .map(|val| Move::from(val.split_at(1)))
        .collect::<Vec<Move>>()
}

fn part_one(tokens: &Vec<Move>) -> i32 {
    let mut dial = 50;
    let mut zero_count = 0;
    for token in tokens {
        dial = match token {
            Move::L(val) => (((dial - val) % 100) + 100) % 100,
            Move::R(val) => (dial + val) % 100,
        };
        if dial == 0 {
            zero_count += 1;
        }
    }
    zero_count
}

fn part_two(tokens: &Vec<Move>) -> i32 {
    let mut dial = 50;
    let mut zero_count = 0;
    for token in tokens {
        dial = match token {
            Move::L(mag) => {
                let new_dial = (100 - dial) % 100;
                zero_count += (new_dial + mag) / 100;
                (((dial - mag) % 100) + 100) % 100
            }
            Move::R(mag) => {
                zero_count += (dial + mag) / 100;
                (dial + mag) % 100
            }
        };
    }
    zero_count
}

pub fn run() {
    let input = include_str!("./input");
    let tokens = parse(input);
    println!("Part A: {}", part_one(&tokens));
    println!("Part B: {}", part_two(&tokens));
}
