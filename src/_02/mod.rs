use std::fmt::Debug;
use std::ops::Range;

fn parse(i: &'static str) -> Vec<Range<i64>> {
    i.split(',')
        .map(|range| {
            let mut l = range.split("-");
            (l.next().unwrap(), l.next().unwrap())
        })
        .map(|(low, high)| low.parse::<i64>().unwrap()..high.parse::<i64>().unwrap() + 1)
        .collect::<Vec<Range<i64>>>()
}

fn valid_part_a(i: i64) -> bool {
    let n_as_str = i.to_string();
    let n_len = n_as_str.len();

    // valid number if its length is not even
    if n_len % 2 != 0 {
        return true;
    }

    let x = n_as_str.split_at(n_len / 2);
    x.0 != x.1
}

fn valid_part_b(inp: i64) -> bool {
    let s = inp.to_string();
    let len = s.len();

    for parts in 2..=len {
        if len % parts != 0 {
            continue;
        }

        let chunk = len / parts;
        let first = &s[0..chunk];
        let mut same = true;

        for i in 1..parts {
            if &s[i * chunk..(i + 1) * chunk] != first {
                same = false;
                break;
            }
        }

        if same {
            return false;
        }
    }

    true
}

fn compute<I, F>(i: I, valid_fn: F) -> i64
where
    I: IntoIterator<Item = Range<i64>>,
    F: Fn(i64) -> bool,
{
    i.into_iter()
        .map(|range| range.filter(|n| !valid_fn(*n)).fold(0, |acc, x| acc + x))
        .fold(0, |acc, x| acc + x)
}

pub fn run() {
    let input = include_str!("./input");
    let ranges = parse(input);
    println!("Part A : {}", compute(ranges.clone(), valid_part_a));
    println!("Part B : {}", compute(ranges, valid_part_b));
}
