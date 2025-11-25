#[derive(Clone, Debug)]
struct Card {
    card_no: usize,
    play_numbers: Vec<usize>,
    winning_numbers: Vec<usize>,
}

pub fn solve(input: &str) -> usize {
    let delimiters = [':', '|'];
    let mut cards: Vec<Card> = vec![];
    for card_line in input.lines() {
        let card: Vec<&str> = card_line.split(|c| delimiters.contains(&c)).collect();
        let card_split: Vec<&str> = card.get(0).unwrap().split_whitespace().collect();
        let card_no: usize = card_split.get(1).unwrap().parse::<usize>().unwrap();
        let play_numbers: Vec<usize> = card
            .get(1)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        let winning_numbers: Vec<usize> = card
            .get(2)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        cards.push(Card {
            card_no,
            play_numbers,
            winning_numbers,
        });
    }
    let mut i = 0;
    while i < cards.len() {
        let mut card_score = 0;
        for play_number in cards[i].play_numbers.iter() {
            if cards[i].winning_numbers.contains(&play_number) {
                card_score += 1;
            }
        }

        if card_score > 0 {
            for new_card_no in 0..card_score {
                cards.push(cards[cards[i].card_no + new_card_no].clone());
            }
        }
        i += 1;
    }


    cards.len()
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
    fn test_part2() {
        assert_eq!(solve(TEST_INPUT), 30);
    }
}
