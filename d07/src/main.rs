use aoc::localpath;

fn parse_input(input: &str) -> anyhow::Result<Vec<i32>> {
    Ok(input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?)
}

fn part2(crabs: &[i32]) -> anyhow::Result<u32> {
    let min = *crabs
        .iter()
        .min()
        .ok_or_else(|| anyhow::anyhow!("no min"))? as i32;
    let max = *crabs
        .iter()
        .max()
        .ok_or_else(|| anyhow::anyhow!("no max"))? as i32;

    (min..=max)
        .map(|location| {
            crabs
                .iter()
                .map(|crab| {
                    (0..=(crab - location).abs() as u32)
                        .reduce(std::ops::Add::add)
                        .unwrap_or(0)
                })
                .sum()
        })
        .min()
        .ok_or_else(|| anyhow::anyhow!("no final min"))
}

fn part1(crabs: &[i32]) -> anyhow::Result<u32> {
    let min = *crabs
        .iter()
        .min()
        .ok_or_else(|| anyhow::anyhow!("no min"))? as i32;
    let max = *crabs
        .iter()
        .max()
        .ok_or_else(|| anyhow::anyhow!("no max"))? as i32;

    (min..=max)
        .map(|location| {
            crabs
                .iter()
                .map(|crab| (crab - location).abs() as u32)
                .sum()
        })
        .min()
        .ok_or_else(|| anyhow::anyhow!("no min"))
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string(localpath!("input.txt"))?;
    let crabs = parse_input(&input)?;
    println!("day 07 part 1: {}", part1(&crabs)?);
    println!("day 07 part 2: {}", part2(&crabs)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_parse_input() {
        let crabs = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(crabs.first().unwrap(), &16_i32);
        assert_eq!(crabs.last().unwrap(), &14_i32);
    }

    #[test]
    fn test_part1() {
        let crabs = vec![8, 8, 10];
        assert_eq!(part1(&crabs).unwrap(), 2);

        let crabs = parse_input(EXAMPLE_INPUT).unwrap();
        let expected = 37;
        assert_eq!(part1(&crabs).unwrap(), expected);
    }

    #[test]
    fn test_part2() {
        let crabs = parse_input(EXAMPLE_INPUT).unwrap();
        let expected = 168;
        assert_eq!(part2(&crabs).unwrap(), expected);
    }
}
