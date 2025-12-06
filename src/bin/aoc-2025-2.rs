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
                let start: u64 = range_ends.next().unwrap().parse().unwrap();
                let end: u64 = range_ends.next().unwrap().parse().unwrap();
                start..=end
            })
            .collect()
    };

    let builder = builder.parser(Box::new(parser));

    fn count_reps_in_range(range: RangeInclusive<u64>) -> u64 {
        // 11..=22
        // both ends even number of digits
        // 2 digits means only patterns of 1 exist
        0
    }

    let solver_1 = |parsed: &Parsed| -> u64 { 0 };

    let (sol_1, sol_2) = builder.solve();
    println!("{sol_1}, {sol_2}");
}
