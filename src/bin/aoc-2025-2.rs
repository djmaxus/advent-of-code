//! 2025 day 2
use advent_of_code::ProblemBuilder;
use std::ops::RangeInclusive;
fn main() {
    type Parsed = Vec<RangeInclusive<u64>>;
    let builder = ProblemBuilder::<Parsed>::default()
        .input(include_str!("../../input/2025-2.txt"))
        .test_input(
            r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#,
        )
        .test_input_parsed(vec![11..=22,95..=115,998..=1012,1188511880..=1188511890,222220..=222224,1698522..=1698528,446443..=446449,38593856..=38593862,565653..=565659,824824821..=824824827,2121212118..=2121212124])
    .test_ans_1(1227775554);

    let parser = |input: &str| -> Parsed {
        input
            .split_terminator(",")
            .map(|range_str: &str| {
                let mut range_ends = range_str.split_terminator("-");
                let start: u64 = range_ends.next().unwrap().trim().parse().unwrap();
                let end: u64 = range_ends.next().unwrap().trim().parse().unwrap();
                start..=end
            })
            .collect()
    };

    let builder = builder.parser(Box::new(parser));

    fn count_reps_in_range(range: &RangeInclusive<u64>) -> u64 {
        let start = *range.start() as u128;
        let end = *range.end() as u128;
        let mut count: u128 = 0;

        // repeated pattern has even number of digits: 2*k digits, k >= 1
        // u64 has at most 20 digits, so k <= 10
        for k in 1..=10 {
            let pow10k = 10u128.pow(k as u32);
            let mul = pow10k + 1; // N = X * (10^k + 1)
            let x_min = if k == 1 {
                1u128
            } else {
                10u128.pow((k - 1) as u32)
            };
            let x_max = 10u128.pow(k as u32) - 1;

            // bounds for X so that N is within [start, end]
            let min_needed = start.div_ceil(mul); // ceil(start / mul)
            let max_allowed = end / mul; // floor(end / mul)

            let lo = std::cmp::max(x_min, min_needed);
            let hi = std::cmp::min(x_max, max_allowed);
            if lo <= hi {
                count += hi - lo + 1;
            }
        }

        count as u64
    }

    let solver_1 = |parsed: &Parsed| -> u64 { parsed.iter().map(count_reps_in_range).sum() };

    let builder = builder.solver_1(Box::new(solver_1));

    let (sol_1, sol_2) = builder.solve();
    println!("{sol_1}, {sol_2}");
}
