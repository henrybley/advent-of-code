#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut data = vec![];

    for line in input.lines() {
        let items = line.split_whitespace();
        let mut items_vec = vec![];
        for item in items {
            items_vec.push(item.parse::<i32>().unwrap());
        }
        data.push(items_vec);
    }

    let result: usize = data
        .iter()
        .map(|report| {
            report
                .windows(2)
                .all(|level| level[0] > level[1] && level[0] - level[1] <= 3)
                || report
                    .windows(2)
                    .all(|level| level[0] < level[1] && level[1] - level[0] <= 3)
        })
        .filter(|s| *s)
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
