use std::{
    num::{NonZeroU64, NonZeroUsize},
    ops::Sub,
};

use advent_of_code::ProblemBuilder;

type Parsed = Vec<Bank>;
type Bank = Vec<Battery>;
type Battery = u64;

fn parser(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10).map(|digit| digit.into()))
                .collect()
        })
        .collect()
}

fn calc_joltage(bank: &Bank) -> u64 {
    largest_number(bank, 2)
        .into_iter()
        .rev()
        .enumerate()
        .map(|(idx, digit)| digit * 10_u64.pow(idx.try_into().unwrap()))
        .sum()
}

fn total_output_joltage(parsed: &Parsed) -> u64 {
    parsed.iter().map(calc_joltage).sum()
}
fn total_output_joltage_12(parsed: &Parsed) -> u64 {
    parsed.iter().map(Vec::as_slice).map(calc_joltage_12).sum()
}

fn calc_joltage_12(bank: &[u64]) -> u64 {
    largest_number(bank, 12)
        .into_iter()
        .rev()
        .enumerate()
        .map(|(idx, digit)| digit * 10_u64.pow(idx.try_into().unwrap()))
        .sum()
}

fn largest_number(seq: &[u64], num_digits: usize) -> Vec<u64> {
    if num_digits == 0 {
        return vec![];
    }
    let num_digits = NonZeroUsize::try_from(num_digits).unwrap();
    // 1. Find the first entry of the largest digit
    let idx_max = seq.iter().enumerate().rev().max_by_key(|(_, el)| *el);
    let (idx, max_digit) = match idx_max {
        None => return vec![], // empty input sequence
        Some((idx, &max_digit)) => (idx, max_digit),
    };

    let left_seq = &seq[0..idx];
    let right_seq = &seq[(idx + 1)..];

    // 3. Calc how many digits are to compute from left and right
    let right_part_len = num_digits.get().sub(1).min(right_seq.len());
    let left_part_len = num_digits.get().sub(1).sub(right_part_len);
    assert_eq!(left_part_len + right_part_len + 1, num_digits.get());
    assert!(left_part_len < num_digits.get());
    // 4. Compute recursively on left and right slices
    let left_sub_num = largest_number(left_seq, left_part_len);
    let right_sub_num = largest_number(right_seq, right_part_len);
    // 5. Merge: [left, this, right] and return
    let mut result = Vec::new();
    result.extend(left_sub_num);
    result.extend([max_digit]);
    result.extend(right_sub_num);
    result
}

fn main() {
    let builder = ProblemBuilder::<Parsed>::default()
        .test_input(
            r#"987654321111111
811111111111119
234234234234278
818181911112111"#,
        )
        .test_ans_1(357)
        .test_input_parsed(vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
            vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
            vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        ])
        .solver_1(Box::new(total_output_joltage))
        .parser(Box::new(parser))
        .input(include_str!("../../input/2025-3.txt"))
        .test_ans_2(3121910778619)
        .solver_2(Box::new(total_output_joltage_12));
    println!("{}", builder);
}
