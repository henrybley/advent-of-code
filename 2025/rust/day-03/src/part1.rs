pub fn solve(input: &str) -> usize {
    let mut result = 0;
    let number_chars = vec!['9', '8', '7', '6', '5', '4', '3', '2', '1'];
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();
        for i in number_chars.iter() {
            if let Some(highest_number_position) = chars.iter().position(|c| *c == *i) {
                
                if highest_number_position + 1 < chars.len() {
                    let sub_chars = &chars[highest_number_position + 1..chars.len()];
                    for i in number_chars.iter() {
                        if let Some(second_highest_number_position) =
                            sub_chars.iter().position(|c| *c == *i)
                        {
                            let joltage = format!(
                                "{}{}",
                                chars.get(highest_number_position).unwrap(),
                                sub_chars.get(second_highest_number_position).unwrap()
                            );
                            result += joltage.parse::<usize>().unwrap();
                            break;
                        }
                    }
                    break;
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT), 357);
    }
}
