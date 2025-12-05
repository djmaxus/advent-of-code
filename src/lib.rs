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
