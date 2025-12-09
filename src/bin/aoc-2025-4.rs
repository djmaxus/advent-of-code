use std::ops::AddAssign;

fn main() {
    let problem = advent_of_code::ProblemBuilder::<Parsed>::default()
        .test_input(
            r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#,
        )
        .test_ans_1(13)
        .parser(Box::new(parser))
        .solver_1(Box::new(solver_1))
        .input(include_str!("../../input/2025-4.txt"));
    println!("{problem}");
}
type Parsed = Vec<Vec<bool>>;

fn parser(input: &str) -> Parsed {
    fn to_vec_bool(input: &str) -> Vec<bool> {
        input
            .chars()
            .map(|char| match char {
                '.' => false,
                '@' => true,
                _ => unreachable!(),
            })
            .collect()
    }
    input.lines().map(to_vec_bool).collect()
}

fn solver_1(parsed: &Parsed) -> u64 {
    let num_rows = parsed.len();
    let num_cols = parsed.get(0).unwrap_or(&vec![]).len();

    let mut counters = vec![vec![0_u64; num_cols]; num_rows];
    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            if !peek(parsed, row_idx, col_idx) {
                continue;
            }
            // eval surrounding indices
            // row_idx [-1,0,1]
            // col_idx [1,0,1]
            // not [0,0]

            for delta_row in [-1_isize, 0, 1] {
                if delta_row == -1 && row_idx == 0 {
                    continue;
                }
                if delta_row == 1 && row_idx == num_rows - 1 {
                    continue;
                }
                for delta_col in [-1_isize, 0, 1] {
                    if delta_row == 0 && delta_col == 0 {
                        continue;
                    }
                    if delta_col == -1 && col_idx == 0 {
                        continue;
                    }
                    if delta_col == 1 && col_idx == num_cols - 1 {
                        continue;
                    }
                    increment(
                        &mut counters,
                        row_idx.saturating_add_signed(delta_row),
                        col_idx.saturating_add_signed(delta_col),
                    );
                }
            }
        }
    }

    let mut count = 0;
    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            if !peek(parsed, row_idx, col_idx) {
                continue;
            }
            if peek(&counters, row_idx, col_idx) < 4 {
                count += 1;
            }
        }
    }
    count
}

fn increment(counters: &mut Vec<Vec<u64>>, row_idx: usize, col_idx: usize) {
    counters
        .get_mut(row_idx)
        .unwrap()
        .get_mut(col_idx)
        .unwrap()
        .add_assign(1);
}

fn peek<Item: Copy>(grid: &Vec<Vec<Item>>, row_idx: usize, col_idx: usize) -> Item {
    *grid.get(row_idx).unwrap().get(col_idx).unwrap()
}
