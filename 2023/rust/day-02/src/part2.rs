use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        if let Some((_, string_sets)) = line.split_once(":") {
            let mut min_cube_counts: HashMap<&str, usize> =
                HashMap::from([("red", 0), ("blue", 0), ("green", 0)]);
            let delimiters = [',', ';'];
            for cubes in string_sets.split(|c| delimiters.contains(&c)) {
                if let Some((count, cube_color)) = cubes.trim().split_once(" ") {
                    let count = count.parse::<usize>().unwrap();
                    if count > *min_cube_counts.get(cube_color).unwrap() {
                        min_cube_counts.insert(cube_color, count);
                    }
                }
            }

            result += min_cube_counts.values().product::<usize>();
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_part2() {
        assert_eq!(solve(TEST_INPUT), 2286);
    }
}
