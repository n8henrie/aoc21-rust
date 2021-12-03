fn parse_input(input: &str) -> anyhow::Result<Vec<u32>> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            Some(line.parse().map_err(Into::into))
        })
        .collect()
}

fn part1<T>(slice: &[T]) -> usize
where
    T: std::cmp::Ord,
{
    slice
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

fn part2<T>(slice: &[T]) -> usize
where
    u32: std::iter::Sum<T>,
    T: Copy,
{
    let sums: Vec<u32> = slice
        .windows(3)
        .map(|window| window.iter().copied().sum())
        .collect();
    part1(&sums)
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let parsed = parse_input(&input)?;
    println!("day 01 part 1: {}", part1(&parsed));
    println!("day 01 part 2: {}", part2(&parsed));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";
    #[test]
    fn test_parse() {
        let output = parse_input(EXAMPLE_INPUT).unwrap();
        let expected_output: Vec<u32> =
            vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_part1() {
        let parsed = parse_input(EXAMPLE_INPUT).unwrap();
        let output = part1(&parsed);
        assert_eq!(output, 7);
    }

    #[test]
    fn test_part2() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part2(&input), 5);
    }
}
