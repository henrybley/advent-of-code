pub fn solve(input: &str) -> usize {
    let mut result = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let line_len = grid.get(0).unwrap().len();
    for (line_no, line) in grid.iter().enumerate() {
        for (pos, pos_contents) in line.iter().enumerate() {
            if pos_contents == &'@' {
                let start_range = if pos == 0 { pos } else { pos - 1 };
                let end_range = if pos == line_len - 1 { pos } else { pos + 1 };
                let current = &line[start_range..=end_range];

                let mut above = None;
                if line_no != 0 {
                    above = Some(&grid.get(line_no - 1).unwrap()[start_range..=end_range]);
                }

                let mut below = None;
                if line_no < grid.len() - 1 {
                    below = Some(&grid.get(line_no + 1).unwrap()[start_range..=end_range]);
                }

                if can_be_removed(current, above, below) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn can_be_removed(current: &[char], above: Option<&[char]>, below: Option<&[char]>) -> bool {
    let max_adjacent = 4;
    let target = '@';

    current
        .iter()
        .chain(above.into_iter().flat_map(|s| s.iter()))
        .chain(below.into_iter().flat_map(|s| s.iter()))
        .filter(|&&c| c == target)
        .count()
        - 1
        < max_adjacent
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT), 13);
    }
}
