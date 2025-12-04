type Battery = u64;
type Bank = Vec<Battery>;

fn parse(i: &'static str) -> Vec<Bank> {
    i.lines()
        .map(|line| {
            line.chars()
                .map(|x| x as Battery - '0' as Battery)
                .collect::<Bank>()
        })
        .collect::<Vec<Bank>>()
}

fn generic_max(banks: &Vec<Bank>, take_count: usize) -> u64 {
    let mut sum: u64 = 0;
    for bank in banks {
        let mut line_sum: u64 = 0;
        let mut start_pos = 0;

        for digit_base in (0..take_count).rev() {
            let end_pos = bank.iter().len() - digit_base;
            let search_slice = &bank[start_pos..end_pos];
            let (skip_index, max) =
                search_slice
                    .iter()
                    .enumerate()
                    .fold((0, &search_slice[0]), |acc, x| {
                        let (_, best_v) = acc;
                        let (i, v) = x;
                        if v > best_v { (i, v) } else { acc }
                    });

            line_sum += (*max as u64) * 10_u64.pow(digit_base as u32);
            start_pos = start_pos + skip_index + 1;
        }
        sum += line_sum;
    }
    sum
}

fn part_a(banks: &Vec<Bank>) -> u64 {
    generic_max(banks, 2)
}

fn part_b(banks: &Vec<Bank>) -> u64 {
    generic_max(banks, 12)
}

pub fn run() {
    let i = include_str!("./input");
    let banks = parse(i);
    println!("Part A: {}", part_a(&banks));
    println!("Part B: {}", part_b(&banks));
}
