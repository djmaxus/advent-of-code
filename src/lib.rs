use std::fmt::Display;

#[macro_export]
macro_rules! aoc_template {
    (
        $input_file:literal,
        $test_input:literal,
        $test_out_1:literal,
        $parsed_input:ty,
        $test_parse:expr,
        $input_name:ident,
        $parser_impl:block,
        $solve_1_impl:block,
        $test_out_2:literal,
        $solve_2_impl:block
    ) => {
        fn main() {
            let input = parse(include_str!($input_file));
            let answer_1 = solve_1(input.clone());
            println!("{}", answer_1.value);

            let answer_2 = solve_2(input);
            println!("{}", answer_2.value);
        }

        #[derive(Clone)]
        pub struct Input {
            pub value: $parsed_input,
        }

        #[derive(Debug)]
        pub struct Answer {
            pub value: u64,
        }

        fn parse($input_name: &str) -> Input {
            $parser_impl
        }

        fn solve_1($input_name: Input) -> Answer {
            $solve_1_impl
        }

        fn solve_2($input_name: Input) -> Answer {
            $solve_2_impl
        }

        #[test]
        pub fn test() {
            const INPUT: &str = $test_input;

            let input = parse(INPUT);
            assert_eq!(input.value, $test_parse);

            const ANSWER_1: u64 = $test_out_1;
            let answer_1 = solve_1(input.clone());
            assert_eq!(answer_1.value, ANSWER_1);

            const ANSWER_2: u64 = $test_out_2;
            let answer_2 = solve_2(input);
            assert_eq!(answer_2.value, ANSWER_2);
        }
    };
}

#[macro_export]
macro_rules! aoc_template_1 {
    (
        $input_file:literal,
        $test_input:literal,
        $test_out_1:literal,
        $parsed_input:ty,
        $test_parse:expr,
        $input_name:ident,
        $parser_impl:block,
        $solve_1_impl:block,
    ) => {
        advent_of_code::aoc_template!(
            $input_file,
            $test_input,
            $test_out_1,
            $parsed_input,
            $test_parse,
            $input_name,
            $parser_impl,
            $solve_1_impl,
            0,
            { Answer { value: 0 } }
        );
    };
}

#[macro_export]
macro_rules! aoc_template_empty {
    (
        $input_file:literal,
        $test_input:literal,
        $test_out_1:literal,
    ) => {
        advent_of_code::aoc_template!(
            $input_file,
            $test_input,
            $test_out_1,
            (),
            (),
            input,
            { Input { value: () } },
            { Answer { value: 0 } },
            0,
            { Answer { value: 0 } }
        );
    };
}

#[derive(Default)]
pub struct ProblemBuilder<Parsed: Default> {
    test_input: Option<String>,
    input: Option<String>,
    test_ans_1: Option<u64>,
    test_ans_2: Option<u64>,
    test_input_parsed: Option<Parsed>,
    parser: Option<Parser<Parsed>>,
    solver_1: Option<Solver<Parsed>>,
    solver_2: Option<Solver<Parsed>>,
}

type Parser<Parsed> = Box<dyn Fn(&str) -> Parsed>;
type Solver<Parsed> = Box<dyn Fn(&Parsed) -> u64>;

impl<Parsed: Default + PartialEq> ProblemBuilder<Parsed> {
    pub fn test_input(mut self, test_input: &str) -> Self {
        self.test_input = Some(test_input.into());
        self
    }

    pub fn test_ans_1(mut self, test_ans_1: u64) -> Self {
        self.test_ans_1 = Some(test_ans_1);
        self
    }

    pub fn test_ans_2(mut self, test_ans_2: u64) -> Self {
        self.test_ans_2 = Some(test_ans_2);
        self
    }

    pub fn test_input_parsed(mut self, test_input_parsed: Parsed) -> Self {
        self.test_input_parsed = Some(test_input_parsed);
        self
    }

    pub fn parser(mut self, parser: Box<dyn Fn(&str) -> Parsed>) -> Self {
        self.parser = Some(parser);
        self
    }

    pub fn solver_1(mut self, solver_1: Box<dyn Fn(&Parsed) -> u64>) -> Self {
        self.solver_1 = Some(solver_1);
        self
    }

    pub fn solver_2(mut self, solver_2: Box<dyn Fn(&Parsed) -> u64>) -> Self {
        self.solver_2 = Some(solver_2);
        self
    }

    pub fn input(mut self, input: &str) -> Self {
        self.input = Some(input.into());
        self
    }
}

pub enum Answer {
    NoData,
    FailedParser,
    FailedSolve,
    TestPassed,
    Ok(u64),
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Answer::NoData => "Not enough data".to_owned(),
            Answer::FailedParser => "Parser test failed".to_owned(),
            Answer::FailedSolve => "Solver test failed".to_owned(),
            Answer::TestPassed => "Test passed".to_owned(),
            Answer::Ok(ans) => ans.to_string(),
        };
        write!(f, "{description}")
    }
}

impl<Parsed: Default + PartialEq> ProblemBuilder<Parsed> {
    fn run(&self, input: Option<&str>) -> (Option<Parsed>, Option<u64>, Option<u64>) {
        // parse and return
        let parsed = self.parser.as_ref().and_then(|parser| input.map(parser));

        // Solve part 1 and return
        let sol_1 = self
            .solver_1
            .as_ref()
            .and_then(|solver_1| parsed.as_ref().map(solver_1));
        // Solve part 2 and return
        let sol_2 = self
            .solver_2
            .as_ref()
            .and_then(|solver_2| parsed.as_ref().map(solver_2));

        (parsed, sol_1, sol_2)
    }

    pub fn solve(&self) -> (Answer, Answer) {
        let (parse_eq, eq_1, eq_2) = self.test();
        // is parsing is wrong
        match parse_eq {
            Some(parse_eq) if !parse_eq => {
                return (Answer::FailedParser, Answer::FailedParser);
            }
            _ => (),
        };

        let mut ans_1 = Answer::NoData;
        match eq_1 {
            Some(eq_1) if !eq_1 => ans_1 = Answer::FailedSolve,
            Some(eq_1) if eq_1 => ans_1 = Answer::TestPassed,
            _ => (),
        };

        let mut ans_2 = Answer::NoData;
        match eq_2 {
            Some(eq_2) if !eq_2 => ans_2 = Answer::FailedSolve,
            Some(eq_2) if eq_2 => ans_2 = Answer::TestPassed,
            _ => (),
        };

        let (_, sol_1, sol_2) = self.run(self.input.as_deref());

        ans_1 = match (sol_1, ans_1) {
            (_, Answer::FailedSolve) => Answer::FailedSolve,
            (None, Answer::TestPassed) => Answer::TestPassed,
            (None, _) => Answer::NoData,
            (Some(ans), _) => Answer::Ok(ans),
        };

        ans_2 = match (sol_2, ans_2) {
            (_, Answer::FailedSolve) => Answer::FailedSolve,
            (None, Answer::TestPassed) => Answer::TestPassed,
            (None, _) => Answer::NoData,
            (Some(ans), _) => Answer::Ok(ans),
        };

        (ans_1, ans_2)
    }

    fn test_case(&self) -> (Option<&Parsed>, Option<u64>, Option<u64>) {
        (
            self.test_input_parsed.as_ref(),
            self.test_ans_1,
            self.test_ans_2,
        )
    }

    pub fn test(&self) -> (Option<bool>, Option<bool>, Option<bool>) {
        let (parsed, sol_1, sol_2) = self.run(self.test_input.as_deref());
        let (parsed_test, sol_1_test, sol_2_test) = self.test_case();

        let parse_eq: Option<bool> = match (parsed, parsed_test) {
            (Some(parsed), Some(parsed_test)) => Some(&parsed == parsed_test),
            _ => None,
        };

        let sol_1_eq: Option<bool> = match (sol_1, sol_1_test) {
            (Some(sol_1), Some(sol_1_test)) => Some(sol_1 == sol_1_test),
            _ => None,
        };

        let sol_2_eq: Option<bool> = match (sol_2, sol_2_test) {
            (Some(sol_2), Some(sol_2_test)) => Some(sol_2 == sol_2_test),
            _ => None,
        };

        (parse_eq, sol_1_eq, sol_2_eq)
    }
}

impl<Parsed: Default + PartialEq> Display for ProblemBuilder<Parsed> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sol = self.solve();
        write!(f, "Part 1: {}\n Part 2: {}", sol.0, sol.1)
    }
}
