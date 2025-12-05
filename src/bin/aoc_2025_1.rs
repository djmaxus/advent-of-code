// 2025 day 1

impl<Parsed: Default + PartialEq> ProblemBuilder<Parsed> {
    pub fn process(self) {
        self.try_solve_1();
        self.try_solve_2();
    }

    fn try_test_parser(&self) -> Option<bool> {
        let Some(parser) = self.parser.as_ref() else {
            println!("Parser is not provided");
            return None;
        };
        let Some(test_input) = self.test_input.as_deref() else {
            println!("No test input is provided");
            return None;
        };
        let Some(test_input_parsed) = self.test_input_parsed.as_ref() else {
            println!("No parsed test input is provided");
            return None;
        };
        Some(&parser(&test_input) == test_input_parsed)
    }

    fn try_test_1(&self) -> Option<bool> {
        self.try_test_parser();
        println!("Test case 1 is not provided");
        None
    }

    fn try_test_2(&self) -> Option<bool> {
        println!("Test case 2 is not provided");
        None
    }

    fn try_solve_1(&self) {
        println!("Solver 1 is not provided");
        match self.try_test_1() {
            None | Some(true) => todo!("attempt solve"),
            Some(false) => println!("Skip part 1"),
        };
    }

    fn try_solve_2(&self) {
        println!("Solver 1 is not provided");
        match self.try_test_2() {
            None | Some(true) => todo!("attempt solve"),
            Some(false) => println!("Skip part 2"),
        };
    }
}

#[derive(Default)]
struct ProblemBuilder<Parsed: Default + PartialEq> {
    test_input: Option<String>,
    input: Option<String>,
    test_ans_1: Option<u64>,
    test_ans_2: Option<u64>,
    test_input_parsed: Option<Parsed>,
    parser: Option<Box<dyn Fn(&str) -> Parsed>>,
    solver_1: Option<Box<dyn Fn(&Parsed) -> u64>>,
    solver_2: Option<Box<dyn Fn(&Parsed) -> u64>>,
}

impl<Parsed: Default + PartialEq> ProblemBuilder<Parsed> {
    fn test_input(mut self, test_input: &str) -> Self {
        self.test_input = Some(test_input.into());
        self
    }

    fn test_ans_1(mut self, test_ans_1: u64) -> Self {
        self.test_ans_1 = Some(test_ans_1);
        self
    }

    fn test_ans_2(mut self, test_ans_2: u64) -> Self {
        self.test_ans_2 = Some(test_ans_2);
        self
    }

    fn test_input_parsed(mut self, test_input_parsed: Parsed) -> Self {
        self.test_input_parsed = Some(test_input_parsed);
        self
    }

    fn parser(mut self, parser: Box<dyn Fn(&str) -> Parsed>) -> Self {
        self.parser = Some(parser);
        self
    }

    fn input(mut self, input: &str) -> Self {
        self.input = Some(input.into());
        self
    }
}

fn main_impl() {
    let input = include_str!("../../input/2025-1.txt").to_string();
    let builder = ProblemBuilder::<Vec<i64>>::default()
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
        .input(&input)
        .test_ans_1(3)
        .test_ans_2(6)
        .test_input_parsed(vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82]);

    fn parser(input: &str) -> Vec<i64> {
        input
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
            .collect()
    }

    let builder = builder.parser(Box::new(parser));

    builder.process();
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
