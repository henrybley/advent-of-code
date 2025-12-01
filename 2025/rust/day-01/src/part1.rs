pub fn solve(input: &str) -> usize {
    let mut result = 0;
    let mut dial_position: i32 = 50;

    for line in input.lines() {
        //print!("{:?}:{:?}", dial_position, line);
        let mut chars = line.chars();
        let direction = chars.next().unwrap();

        let rest: Vec<char> = chars.collect();
        let amount_string: String = rest[rest.len().saturating_sub(2)..].iter().collect();
        let amount: i32 = amount_string.parse().unwrap();

        //println!(" - {:?}:{:?}", direction, amount);

        if direction == 'L' {
            dial_position -= amount;
        } else {
            dial_position += amount;
        }

        if dial_position < 0 {
            dial_position = dial_position + 100;
        }

        if dial_position > 100 {
            dial_position = dial_position - 100;
        }

        if dial_position == 100 {
            dial_position = 0;
        }

        if dial_position == 0 {
            result += 1
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT), 3);
    }
}
