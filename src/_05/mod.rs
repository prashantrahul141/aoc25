use std::ops::{RangeBounds, RangeInclusive};

fn parse(i: &'static str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (raw_ranges, raw_list) = i.split_once("\n\n").unwrap();
    let ranges = raw_ranges
        .lines()
        .map(|range_str| range_str.split_once("-").unwrap())
        .map(|(lower, upper)| {
            (
                lower.parse::<usize>().unwrap(),
                upper.parse::<usize>().unwrap(),
            )
        })
        .map(|(lower, upper)| lower..=upper)
        .collect::<Vec<_>>();

    let ids = raw_list
        .lines()
        .map(|raw_id| raw_id.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (ranges, ids)
}

fn contained<I, T>(ranges: &Vec<I>, val: &T) -> bool
where
    I: RangeBounds<T>,
    T: PartialOrd,
{
    ranges.iter().any(|ra| ra.contains(val))
}

fn part_a(ranges: &Vec<RangeInclusive<usize>>, ids: Vec<usize>) -> usize {
    ids.iter().filter(|val| contained(&ranges, val)).count()
}

fn collapse(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    ranges.sort_by_key(|r| *r.start());
    let mut merged: Vec<RangeInclusive<usize>> = Vec::new();
    for r in ranges {
        if let Some(last) = merged.last() {
            if *r.start() <= *last.end() {
                let new_range = *last.start()..=*r.end().max(last.end());
                merged.pop();
                merged.push(new_range);
            } else {
                merged.push(r.clone());
            }
        } else {
            merged.push(r.clone());
        }
    }

    merged
}
fn part_b(ranges: Vec<RangeInclusive<usize>>) -> usize {
    collapse(ranges).into_iter().map(|r| r.count()).sum()
}

pub fn run() {
    let i = include_str!("./input");
    let (ranges, ids) = parse(i);
    println!("Part A: {}", part_a(&ranges, ids));
    println!("Part B: {}", part_b(ranges));
}
