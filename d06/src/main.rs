use aoc::localpath;
use std::collections::HashMap;

fn recurse(
    days_left: u32,
    reproduce_in: u8,
    cache: &mut HashMap<u32, u64>,
) -> u64 {
    if let Some(val) = cache.get(&days_left) {
        return *val;
    }
    if reproduce_in == 0 {
        let result = recurse(days_left.saturating_sub(7), 0, cache)
            + recurse(days_left.saturating_sub(9), 0, cache);
        cache.insert(days_left, result);
        return result;
    }
    recurse(days_left.saturating_sub(reproduce_in.into()), 0, cache)
}

fn solve(fish: &[u8], days: u32, cache: &mut HashMap<u32, u64>) -> u64 {
    cache.insert(0, 1);
    fish.iter().map(|&f| recurse(days, f, cache)).sum()
}

fn parse_input(input: &str) -> anyhow::Result<Vec<u8>> {
    Ok(input
        .trim()
        .split(',')
        .map(str::parse::<u8>)
        .collect::<Result<_, _>>()?)
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string(localpath!("input.txt"))?;
    let fish = parse_input(&input)?;
    let mut cache = HashMap::new();
    println!("day 06 part 1: {}", solve(&fish, 80, &mut cache));
    println!("day 06 part 2: {}", solve(&fish, 256, &mut cache));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_parse_input() {
        let fish = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(fish.first().unwrap(), &3_u8);
        assert_eq!(fish.last().unwrap(), &2_u8);
    }

    #[test]
    fn test_solve() {
        let mut cache = HashMap::new();
        let fish = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(solve(&fish, 18, &mut cache), 26);
        assert_eq!(solve(&fish, 80, &mut cache), 5934);
        assert_eq!(solve(&fish, 256, &mut cache), 26984457539);
    }
}
