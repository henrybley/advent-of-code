pub fn solve(input: &str) -> usize {
    let mut result = 0;
    let bank_size = 12;
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        let final_bank = get_digits(chars, bank_size);
        let final_joltage: String = final_bank.iter().collect();

        result += final_joltage.parse::<usize>().unwrap();
    }
    result
}

fn get_digits(mut chars: Vec<char>, len: usize) -> Vec<char> {
    let mut highest_position = 0;
    let mut highest_value = chars.get(0).unwrap().to_digit(10).unwrap();
    
    for i in 1..(chars.len() - len + 1) {
        if let Some(number) = chars.get(i).unwrap().to_digit(10) {
            if number > highest_value {
                highest_value = number;
                highest_position = i;
            }
        }
    }

    chars = chars[highest_position..].to_vec();


    if chars.len() > len {
        if len == 1 {
            chars = vec![chars[0]];
        } else {
            chars = std::iter::once(chars[0])
                .chain(get_digits(chars[1..].to_vec(), len - 1))
                .collect::<Vec<_>>();
        }
    }

    chars
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;

    #[test]
    fn test_part2() {
        assert_eq!(solve(TEST_INPUT), 3121910778619);
    }
}
