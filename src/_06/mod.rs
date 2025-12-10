use std::{u64, usize};

#[derive(Debug)]
enum Op {
    Add,
    Mul,
}

impl TryFrom<char> for Op {
    type Error = ();
    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '+' => Ok(Op::Add),
            '*' => Ok(Op::Mul),
            _ => Err(()),
        }
    }
}

fn parse(i: &'static str) -> (Vec<Op>, Vec<Vec<&'static str>>) {
    let lines_count = i.lines().count();
    let items_count = i.lines().next().unwrap().split_whitespace().count();

    // get ops
    let mut ops = Vec::with_capacity(items_count);
    let mut ops_indices = Vec::with_capacity(items_count);
    let ops_line = i.lines().last().unwrap();
    for (char_index, char_val) in ops_line.chars().enumerate() {
        if let Ok(ty) = Op::try_from(char_val) {
            ops.push(ty);
            ops_indices.push(char_index);
        }
    }

    let line_length = i.lines().next().unwrap().len();
    ops_indices.push(line_length);

    // parse values
    let mut vals = Vec::with_capacity(lines_count - 1);
    vals.extend(
        (0..items_count)
            .map(|_| Vec::with_capacity(items_count))
            .collect::<Vec<_>>(),
    );

    // iterator over indices of ops
    for line in i.lines().take(lines_count - 1) {
        // iterator over indices of ops
        for (start_end_index, start_end) in ops_indices.windows(2).enumerate() {
            let val_index_range = start_end[0]
                ..start_end[1]
                    - if start_end_index != lines_count - 1 {
                        1
                    } else {
                        0
                    };
            vals[start_end_index].push(&line[val_index_range]);
        }
    }

    dbg!(vals.len());

    (ops, vals)
}

fn part_a(ops: &Vec<Op>, vals: &Vec<Vec<&'static str>>) -> i64 {
    let mut total = 0;
    for (val_index, val) in vals.into_iter().enumerate() {
        total += match ops[val_index] {
            Op::Add => val
                .iter()
                .fold(0, |acc, next| acc + next.trim().parse::<i64>().unwrap()),
            Op::Mul => val
                .iter()
                .fold(1, |acc, next| acc * next.trim().parse::<i64>().unwrap()),
        };
    }
    total
}

fn parse_part_b(input: &str) -> (Vec<Op>, Vec<Vec<u64>>) {
    let mut lines: Vec<&str> = input.lines().collect();
    while lines.last().map(|l| l.trim().is_empty()).unwrap_or(false) {
        lines.pop();
    }

    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut padded: Vec<Vec<char>> = lines
        .iter()
        .map(|l| {
            let mut v: Vec<char> = l.chars().collect();
            v.resize(width, ' ');
            v
        })
        .collect();

    let ops_row = padded.pop().unwrap();
    let num_rows = padded;

    let mut ops = Vec::new();
    let mut values = Vec::new();

    let mut col = width as isize - 1;
    while col >= 0 {
        while col >= 0 {
            let op_char = ops_row[col as usize];
            let mut all_space = true;
            for r in &num_rows {
                if r[col as usize] != ' ' {
                    all_space = false;
                    break;
                }
            }
            if op_char == ' ' && all_space {
                col -= 1;
            } else {
                break;
            }
        }
        if col < 0 {
            break;
        }

        let block_end = col;
        let mut block_start = col;
        while block_start >= 0 {
            let op_char = ops_row[block_start as usize];
            let mut has_content = false;
            for r in &num_rows {
                if r[block_start as usize] != ' ' {
                    has_content = true;
                    break;
                }
            }
            if has_content || op_char == '+' || op_char == '*' {
                block_start -= 1;
            } else {
                break;
            }
        }
        block_start += 1;

        let op = match ops_row[block_start as usize] {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => unreachable!(),
        };
        ops.push(op);

        let mut nums = Vec::new();
        for c in (block_start..=block_end).rev() {
            let mut s = String::new();
            for r in &num_rows {
                let ch = r[c as usize];
                if ch != ' ' {
                    s.push(ch);
                }
            }
            if !s.is_empty() {
                nums.push(s.parse::<u64>().unwrap());
            }
        }
        values.push(nums);

        col = block_start - 1;
    }

    (ops, values)
}

fn part_b(ops: &[Op], values: &[Vec<u64>]) -> u64 {
    let mut total = 0;
    for (op, nums) in ops.iter().zip(values.iter()) {
        let v: u64 = match op {
            Op::Add => nums.iter().copied().sum(),
            Op::Mul => nums.iter().copied().product(),
        };
        total += v;
    }
    total
}

pub fn run() {
    let i = include_str!("./input");
    let (ops, vals) = parse(i);
    println!("Part A: {}", part_a(&ops, &vals));

    let (ops, vals) = parse_part_b(i);
    println!("Part B: {}", part_b(&ops, &vals));
}
