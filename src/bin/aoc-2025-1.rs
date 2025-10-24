/// 2025 day 1
fn main() {
    let input = parse(include_str!("../../input/2025-1.txt"));
    let answer_1 = solve_1(input.clone());
    println!("{}", answer_1.value);

    let answer_2 = solve_2(input);
    println!("{}", answer_2.value);
}

#[derive(Clone)]
pub struct Input {
    pub value: (Vec<u32>, Vec<u32>),
}

#[derive(Debug)]
pub struct Answer {
    pub value: u64,
}

fn parse(input: &str) -> Input {
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
}

fn solve_1(input: Input) -> Answer {
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
}

fn solve_2(input: Input) -> Answer {
    let (left, right) = input.value;
    let mut n_left_in_right: Vec<usize> = vec![];
    for &id in left.iter() {
        let count = right.iter().filter(|&&item_right| item_right == id).count();
        n_left_in_right.push(count);
    }

    let value = left
        .into_iter()
        .zip(n_left_in_right)
        .fold(0u64, |acc, (left, n_in_right)| {
            acc + u64::from(left) * u64::try_from(n_in_right).unwrap()
        });
    Answer { value }
}

#[cfg(test)]
pub fn test() {
    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    let input = parse(INPUT);
    assert_eq!(input.value, (vec![4, 2, 1, 3, 3], vec![3, 5, 3, 9, 3]));

    const ANSWER_1: u64 = 11;
    let answer_1 = solve_1(input.clone());
    assert_eq!(answer_1.value, ANSWER_1);

    const ANSWER_2: u64 = 31;
    let answer_2 = solve_2(input);
    assert_eq!(answer_2.value, ANSWER_2);
}
