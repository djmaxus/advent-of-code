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
    let max_with_pos = bank.iter().enumerate().rev().max_by_key(|(_, el)| **el);
    let mut first_digit = None;
    let mut second_digit = None;
    let pos_max: usize = match max_with_pos {
        Some((pos, &val)) => {
            if pos == (bank.len() - 1) {
                second_digit = Some(val); // last battery can only provide the second digit
            } else {
                first_digit = Some(val); // first digit is the largest in the bank unless it's the last
            };
            Some(pos)
        }
        None => return 0, // empty bank
    }
    .unwrap();

    if first_digit.is_some() {
        second_digit = bank.iter().skip(pos_max + 1).max().copied();
    } else if pos_max == (bank.len() - 1) {
        first_digit = bank.iter().take(pos_max).max().copied();
    } else {
        unreachable!("unreachable");
    }
    println!("{}-{}", first_digit.unwrap(), second_digit.unwrap());
    return first_digit.unwrap() * 10 + second_digit.unwrap();
}

fn total_output_joltage(parsed: &Parsed) -> u64 {
    parsed.iter().map(calc_joltage).sum()
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
        .input(include_str!("../../input/2025-3.txt"));
    println!("{}", builder);
}
