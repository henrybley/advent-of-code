pub fn solve(input: &str) -> usize {
    let mut result = 0;
    let mut dial_position: i32 = 50;

    for line in input.lines() {
        let init_dial_position = dial_position;
        print!("{:?}:{:?}", dial_position, line);
        let mut chars = line.chars();
        let direction = chars.next().unwrap();

        let amount_string: String = chars.collect();
        let full_amount: i32 = amount_string.parse().unwrap();

        let amount = full_amount % 100;
        let mut full_loops = (full_amount - amount ) / 100;

        print!(" - {:?}:{:?}:{:?}", direction, full_loops, amount);

        if direction == 'L' {
            dial_position -= amount;
        } else {
            dial_position += amount;
        }

        if dial_position < 0 {
            if init_dial_position != 0 {
                full_loops += 1;
            }
            dial_position = dial_position + 100;
        }

        if dial_position == 0 {
            full_loops += 1;
        }

        if dial_position >= 100 {
            full_loops += 1;
            dial_position = dial_position - 100;
        }

        println!(" - {:?}", full_loops);
        result += full_loops;
    }

    result as usize
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
        assert_eq!(solve(TEST_INPUT), 6);
    }
}
