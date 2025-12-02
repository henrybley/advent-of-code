
pub fn solve(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        for range in line.split(',') {
            if let Some((start_range_string, end_range_string)) = range.split_once("-") {
                let start_range: usize = start_range_string.parse().unwrap();
                let end_range: usize = end_range_string.parse().unwrap();
                for number in start_range..end_range + 1 {
                    let number_string: String = format!("{}", number);
                    for index in 1..number_string.chars().count()/2 + 1 {
                        let number_check = &number_string[0..index];
                        let replaced = number_string.replace(number_check, "");
                        if replaced.len() == 0 {
                            result += number_string.parse::<usize>().unwrap();
                            break;
                        }
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

    const TEST_INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

    #[test]
    fn test_part2() {
        assert_eq!(solve(TEST_INPUT), 4174379265);
    }
}
