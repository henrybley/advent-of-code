pub fn solve(input: &str) -> usize {
    let mut result = 0;
    let delimiters = [':', '|'];
    for line in input.lines() {
        let card: Vec<&str> = line.split(|c| delimiters.contains(&c)).collect();
        let play_numbers: Vec<&str> = card.get(1).unwrap().split_whitespace().collect();
        let winning_numbers: Vec<&str> = card.get(2).unwrap().split_whitespace().collect();

        let mut card_score = 0;
        for play_number in play_numbers {
            if winning_numbers.contains(&play_number) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score = card_score*2;
                }
            }
        }
        result += card_score;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT), 13);
    }
}
