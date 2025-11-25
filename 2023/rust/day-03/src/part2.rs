pub fn solve(input: &str) -> usize {
    let result = 0;

    let lines: Vec<&str> = input.lines().collect();
    for (line_no, line) in lines.iter().enumerate() {
        if let Some(star_index) = line.find('*') {
            println!("{}", star_index);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_part2() {
        assert_eq!(solve(TEST_INPUT), 467835);
    }
}
