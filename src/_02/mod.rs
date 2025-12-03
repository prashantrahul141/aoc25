use std::cmp::max;
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
    fn split_n_parts(parent: &String, n: usize) -> Vec<String> {
        let str_len = parent.len();
        // str cannot be divided into n parts
        if str_len % n != 0 {
            return Vec::new();
        }
        parent
            .chars()
            .collect::<Vec<_>>()
            .chunks(str_len / n)
            .map(|sub| sub.iter().collect::<String>())
            .collect::<Vec<_>>()
    }

    fn vec_all_same<T: Eq + Debug>(vec: Vec<T>) -> bool {
        if vec.is_empty() || vec.len() == 1 {
            return false;
        }

        let first = &vec[0];
        vec.iter().all(|n| n == first)
    }

    let i_str = inp.to_string();
    let mut result = true;
    for i in 2..=i_str.len() {
        if vec_all_same(split_n_parts(&i_str, i)) {
            result = false;
            break;
        }
    }

    result
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
