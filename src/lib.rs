/// Variable parameters:
/// 1. [x] Input file path
/// 3. [x] Parsed input type
/// 4. [x] parser implementation
/// 5. [ ] solvers implementation
/// 6. [ ] test input and answers

macro_rules! aoc_template {
    (
        $input_file:path,
        $test_input:literal,
        $test_out_1:literal,
        $parsed_input:ty,
        $test_parse:expr,
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

        fn parse(input: &str) -> Input {
            $parser_impl
        }

        fn solve_1(input: Input) -> Answer {
            $solve_1_impl
        }

        fn solve_2(input: Input) -> Answer {
            $solve_2_impl
        }

        #[cfg(test)]
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
