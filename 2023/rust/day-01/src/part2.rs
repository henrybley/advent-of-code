use std::collections::HashMap;

use regex::Regex;

pub fn solve(input: &str) -> usize {
    let map = word_to_digit_map();
    let mut sum: usize = 0;
    let re = Regex::new(r"\d").unwrap();

    for line in input.lines() {
        let replaced = replace_words_with_digits(line, &map);
        let numbers: Vec<_> = re.find_iter(&replaced).map(|m| m.as_str()).collect();
        println!("{}", numbers.concat());
        if let Some(first) = numbers.first() {
            if let Some(last) = numbers.last() {
                let num = format!("{}{}", first, last);
                println!("{}", num);
                sum += num.parse::<usize>().unwrap();
            }
        }
    }
    sum
}

fn word_to_digit_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    map.insert("sixteen", "16");
    map.insert("fifteen", "15");
    map.insert("fourteen", "14");
    map.insert("thirteen", "13");
    map.insert("twelve", "12");
    map.insert("eleven", "11");
    map.insert("ten", "10");
    map.insert("nine", "9");
    map.insert("eight", "8");
    map.insert("seven", "7");
    map.insert("six", "6");
    map.insert("five", "5");
    map.insert("four", "4");
    map.insert("three", "3");
    map.insert("two", "2");
    map.insert("one", "1");
    map.insert("zero", "0");
    map
}

fn replace_words_with_digits(s: &str, map: &HashMap<&str, &str>) -> String {
    let mut result = s.to_string();

    for (word, digit) in map {
        result = result.replace(word, digit);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT), 142);
    }
}
