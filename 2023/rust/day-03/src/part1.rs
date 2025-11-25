use std::usize;

pub fn solve(input: &str) -> usize {
    let mut result = 0;
    let lines: Vec<&str> = input.lines().collect();
    for (line_no, line) in lines.iter().enumerate() {
        let mut numbers: Vec<char> = vec![];
        let mut number_positions: Vec<usize> = vec![];
        let line_chars: Vec<char> = line.chars().collect();
        for (index, character) in line_chars.iter().enumerate() {
            if character.is_numeric() {
                numbers.push(*character);
                number_positions.push(index);
            }
            if index + 1 >= line_chars.len() || !line_chars.get(index + 1).unwrap().is_numeric() {
                if !numbers.is_empty() {
                    let mut symbols_check: Vec<char> = vec![];
                    if let Some(next_char) = line_chars.get(index + 1) {
                        symbols_check.push(*next_char);
                    }
                    if let Some(prev_char_position) = number_positions.get(0) {
                        if *prev_char_position > 0 {
                            if let Some(prev_char) = line_chars.get(prev_char_position - 1) {
                                symbols_check.push(*prev_char);
                            }
                        }
                    }
                    if line_no > 0 {
                        if let Some(prev_line) = lines.get(line_no - 1) {
                            let prev_line_chars: Vec<char> = prev_line.chars().collect();
                            for (index, pos) in number_positions.iter().enumerate() {
                                if index == 0 && *pos > 0 {
                                    if let Some(symbol_to_check) = prev_line_chars.get(pos - 1) {
                                        symbols_check.push(*symbol_to_check);
                                    }
                                }
                                if let Some(symbol_to_check) = prev_line_chars.get(*pos) {
                                    symbols_check.push(*symbol_to_check);
                                }
                                if index + 1 == number_positions.len() {
                                    if let Some(symbol_to_check) = prev_line_chars.get(pos + 1) {
                                        symbols_check.push(*symbol_to_check);
                                    }
                                }
                            }
                        }
                    }

                    if let Some(next_line) = lines.get(line_no + 1) {
                        let next_line_chars: Vec<char> = next_line.chars().collect();
                        for (index, pos) in number_positions.iter().enumerate() {
                            if index == 0 && *pos > 0 {
                                if let Some(symbol_to_check) = next_line_chars.get(pos - 1) {
                                    symbols_check.push(*symbol_to_check);
                                }
                            }
                            if let Some(symbol_to_check) = next_line_chars.get(*pos) {
                                symbols_check.push(*symbol_to_check);
                            }
                            if index + 1 == number_positions.len() {
                                if let Some(symbol_to_check) = next_line_chars.get(pos + 1) {
                                    symbols_check.push(*symbol_to_check);
                                }
                            }
                        }
                    }

                    let mut is_part_no = false;
                    for symbol in symbols_check {
                        if !symbol.is_alphabetic() && symbol != '.' {
                            is_part_no = true;
                            break;
                        }
                    }

                    if is_part_no {
                        let number_string: String = numbers.iter().collect();
                        result += number_string.parse::<usize>().unwrap();
                    }
                    numbers = vec![];
                    number_positions = vec![];
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT), 4361);
    }
}
