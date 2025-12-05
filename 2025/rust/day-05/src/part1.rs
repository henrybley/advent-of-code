pub fn solve(input: &str) -> usize {
    let mut result = 0;
    let mut ranges = vec![];
    for line in input.lines() {
        if line.contains('-') {
            if let Some((range_start, range_end)) = line.split_once('-') {
                ranges.push(range_start.parse::<usize>().unwrap()..=range_end.parse::<usize>().unwrap());
            }
        } else if line.trim() != "" {
            if let Ok(food_id) = line.parse::<usize>() {
                for range in ranges.iter() {
                    if range.contains(&food_id) {
                        result += 1;
                        break;
                    }

                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT), 3);
    }
}
