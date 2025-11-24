use regex::Regex;

pub fn solve(input: &str) -> usize {
    let mut sum: usize = 0;
    let re = Regex::new(r"\d").unwrap();

    for line in input.lines() {
        let numbers: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        if let Some(first) = numbers.first() {
            if let Some(last) = numbers.last() {
                let num = format!("{}{}", first, last);
                sum += num.parse::<usize>().unwrap();
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT), 142);
    }
}
