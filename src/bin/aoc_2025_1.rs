// 2025 day 1

use std::path::PathBuf;

struct Problem {
    test_parse: Option<TokenStream>,
    test_1: Option<TokenStream>,
    test_2: Option<TokenStream>,
    solve_1: Option<TokenStream>,
    solve_2: Option<TokenStream>,
}

impl Problem {
    fn builder<Parsed: Default>() -> ProblemBuilder<Parsed> {
        ProblemBuilder::default()
    }
}
use proc_macro2::TokenStream;

#[derive(Default)]
struct ProblemBuilder<Parsed: Default> {
    test_input: Option<String>,
    input_path: Option<PathBuf>,
    test_ans_1: Option<u64>,
    test_ans_2: Option<u64>,
    test_input_parsed: Option<Parsed>,
    solver_1: Option<TokenStream>,
    solver_2: Option<TokenStream>,
}

impl<Parsed: Default> ProblemBuilder<Parsed> {
    fn test_input(mut self, test_input: &str) -> Self {
        self.test_input = Some(test_input.into());
        self
    }
    fn try_build(self) -> Option<Problem> {
        None
    }
}

fn test() -> Option<Problem> {
    Problem::builder::<()>()
        .test_input(
            r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#,
        )
        .try_build()
}

advent_of_code::aoc_template!(
    "../../input/2025-1.txt",
    r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#,
    3,
    Vec<i64>,
    vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82],
    input,
    {
        let value = input
            .lines()
            .map(|line| {
                let dir: char = line.get(0..1).unwrap().parse().unwrap();
                let dir: i64 = match dir {
                    'L' => -1,
                    'R' => 1,
                    _ => panic!(),
                };
                let num: i64 = line.get(1..).unwrap().parse().unwrap();
                num * dir
            })
            .collect();
        Input { value }
    },
    {
        let mut num_zeros = 0;
        input.value.iter().fold(50, |pos, rotation| {
            let new_pos = (pos + rotation) % 100;
            if new_pos == 0 {
                num_zeros += 1;
            }
            new_pos
        });
        Answer { value: num_zeros }
    },
    6,
    {
        let mut num_zeros: u64 = 0;
        input.value.iter().fold(50, |pos: i64, &rotation| {
            let num_full_rot: u64 = rotation.unsigned_abs() / 100;
            num_zeros += num_full_rot;
            let rotation_rem = rotation % 100;
            let new_pos = (pos + rotation_rem).rem_euclid(100);
            match rotation_rem {
                0 => (/* no more rotations */),
                _ if pos == 0 => (/* started at zero, so no more passages */),
                _ if new_pos == 0 => num_zeros += 1,
                i64::MIN..0 if new_pos > pos => num_zeros += 1,
                1..=i64::MAX if new_pos < pos => num_zeros += 1,
                _ => (/* did not cross zero*/),
            }
            new_pos
        });
        Answer { value: num_zeros }
    }
);
