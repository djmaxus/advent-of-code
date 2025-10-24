// 2025 day 2

advent_of_code::aoc_template_1!(
    "../../input/2025-1.txt",
    r#"3   4
4   3
2   5
1   3
3   9
3   3"#,
    11,
    (Vec<u32>, Vec<u32>),
    (vec![4, 2, 1, 3, 3], vec![3, 5, 3, 9, 3]),
    input,
    {
        input.lines().fold(
            Input {
                value: (Vec::new(), Vec::new()),
            },
            |mut acc, line| {
                let mut parts = line.split_whitespace();
                let a: u32 = parts.next().unwrap().parse().unwrap();
                let b: u32 = parts.next().unwrap().parse().unwrap();
                acc.value.0.push(a);
                acc.value.1.push(b);
                acc
            },
        )
    },
    {
        let input_sorted = {
            let mut input = input;
            input.value.0.sort();
            input.value.1.sort();
            input
        };
        let mut value: u64 = 0;
        for (left, right) in input_sorted.value.0.into_iter().zip(input_sorted.value.1) {
            value += u64::from(left.abs_diff(right));
        }
        Answer { value }
    },
);
