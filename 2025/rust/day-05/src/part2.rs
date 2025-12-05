pub fn solve(input: &str) -> usize {
    let mut result = 0;
    let mut ranges = vec![];
    for line in input.lines() {
        if let Some((range_start, range_end)) = line.split_once('-') {
            ranges
                .push(range_start.parse::<usize>().unwrap()..=range_end.parse::<usize>().unwrap());
        } else {
            break;
        }
    }
    ranges.sort_unstable_by_key(|r| *r.start());

    let mut start_number = 0;
    for range in ranges {
        if start_number > *range.end() {
            continue;
        }
        if start_number < *range.start() {
            start_number = *range.start();
        }


        result += range.end()+1 - start_number;
        start_number = range.end() + 1;
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
        assert_eq!(solve(TEST_INPUT), 14);
    }
}
