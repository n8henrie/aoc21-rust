use anyhow::bail;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}
struct Directions(Vec<Direction>);

impl FromStr for Directions {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        Ok(Directions(
            s.lines()
                .map(FromStr::from_str)
                .collect::<anyhow::Result<Vec<_>>>()?,
        ))
    }
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        Ok(
            match s.trim().split_whitespace().collect::<Vec<_>>()[0..2] {
                ["forward", amount] => Forward(amount.parse()?),
                ["up", amount] => Up(amount.parse()?),
                ["down", amount] => Down(amount.parse()?),
                _ => bail!("Couldn't parse line {}", s),
            },
        )
    }
}

impl Directions {
    fn iter(&self) -> impl Iterator<Item = &Direction> {
        self.0.iter()
    }
}

fn part1(directions: &Directions) -> i32 {
    let final_pos =
        directions
            .iter()
            .fold((0, 0), |(x, y), direction| match direction {
                Direction::Forward(amount) => (x + amount, y),
                Direction::Up(amount) => (x, y - amount),
                Direction::Down(amount) => (x, y + amount),
            });
    final_pos.0 * final_pos.1
}

fn part2(directions: &Directions) -> i32 {
    let final_pos =
        directions.iter().fold((0, 0, 0), |(x, y, aim), direction| {
            match direction {
                Direction::Forward(amount) => {
                    (x + amount, y + aim * amount, aim)
                }
                Direction::Up(amount) => (x, y, aim - amount),
                Direction::Down(amount) => (x, y, aim + amount),
            }
        });
    final_pos.0 * final_pos.1
}

fn main() -> anyhow::Result<()> {
    let directions: Directions = std::fs::read_to_string(format!(
        "{}/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?
    .parse()?;
    println!("day 02 part 1: {}", part1(&directions));
    println!("day 02 part 2: {}", part2(&directions));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_parse() {
        let directions: Directions = EXAMPLE_INPUT.parse().unwrap();
        let first = Direction::Forward(5);
        let last = Direction::Forward(2);
        assert_eq!(directions.0.first().unwrap(), &first);
        assert_eq!(directions.0.last().unwrap(), &last);
    }

    #[test]
    fn test_part1() {
        let directions: Directions = EXAMPLE_INPUT.parse().unwrap();
        let result = part1(&directions);
        assert_eq!(result, 150);
    }

    #[test]
    fn test_part2() {
        let directions: Directions = EXAMPLE_INPUT.parse().unwrap();
        let result = part2(&directions);
        assert_eq!(result, 900);
    }
}
