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

    // Count all numbers inside `range` that are formed by repeating a k-digit block twice:
    // numbers of the form N = X concatenated with X. If X has k digits, then
    // N = X * (10^k) + X = X * (10^k + 1).
    // We iterate over possible k (number of digits in X), find all X such that
    // N is within [start, end], and sum those N values.
    fn count_reps_in_range(range: &RangeInclusive<u64>) -> u64 {
        // Convert bounds to u128 so we can safely do multiplication and pow without overflow.
        let start = *range.start() as u128;
        let end = *range.end() as u128;

        // Accumulate total as u128 to avoid intermediate overflow; we'll cast down at the end.
        let mut total: u128 = 0;

        // Repeated pattern has even number of digits: 2*k digits with k >= 1.
        // u64 numbers have at most 20 digits, so k <= 10 (because 2*k <= 20).
        for k in 1..=10 {
            // pow10k = 10^k
            let pow10k = 10u128.pow(k as u32);

            // mul = 10^k + 1, so N = X * mul
            let mul = pow10k + 1;

            // Determine valid range for X (k-digit numbers).
            // If k == 1 then X can start at 1 (single-digit numbers don't include 0 as leading digit).
            // Otherwise X_min = 10^(k-1). X_max = 10^k - 1.
            let x_min = if k == 1 {
                1u128
            } else {
                10u128.pow((k - 1) as u32)
            };
            let x_max = pow10k - 1;

            // For N = X * mul to be within [start, end], X must be within:
            // ceil(start / mul) <= X <= floor(end / mul).
            // Compute ceil(start / mul) as (start + mul - 1) / mul.
            let min_needed = (start + mul - 1) / mul; // ceil division
            let max_allowed = end / mul; // floor division

            // Intersect with the k-digit bounds for X.
            let lo = std::cmp::max(x_min, min_needed);
            let hi = std::cmp::min(x_max, max_allowed);

            // If there is at least one valid X in [lo, hi], count them and add their contributions.
            if lo <= hi {
                // Number of valid X values.
                let cnt = hi - lo + 1;

                // Sum of consecutive integers from lo to hi: (lo + hi) * cnt / 2
                let sum_x = (lo + hi) * cnt / 2;

                // Each X contributes N = X * mul, so total contribution for this k is mul * sum_x.
                total += mul * sum_x;
            }
            // If lo > hi, there are no k-digit repeated numbers in this range; continue to next k.
        }

        // Cast back to u64 for the function return type (caller expects u64).
        total as u64
    }

    let solver_1 = |parsed: &Parsed| -> u64 { parsed.iter().map(count_reps_in_range).sum() };

    let builder = builder.solver_1(Box::new(solver_1));

    let (sol_1, sol_2) = builder.solve();
    println!("{sol_1}, {sol_2}");
}
