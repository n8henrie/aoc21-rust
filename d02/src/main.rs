// --- Day 2: Dive! ---
//
// Now, you need to figure out how to pilot this thing.
//
// It seems like the submarine can take a series of commands like forward 1, down 2, or up 3:
//
//     forward X increases the horizontal position by X units.
//     down X increases the depth by X units.
//     up X decreases the depth by X units.
//
// Note that since you're on a submarine, down and up affect your depth, and so they have the opposite result of what you might expect.
//
// The submarine seems to already have a planned course (your puzzle input). You should probably figure out where it's going. For example:
//
// forward 5
// down 5
// forward 8
// up 3
// down 8
// forward 2
//
// Your horizontal position and depth both start at 0. The steps above would then modify them as follows:
//
//     forward 5 adds 5 to your horizontal position, a total of 5.
//     down 5 adds 5 to your depth, resulting in a value of 5.
//     forward 8 adds 8 to your horizontal position, a total of 13.
//     up 3 decreases your depth by 3, resulting in a value of 2.
//     down 8 adds 8 to your depth, resulting in a value of 10.
//     forward 2 adds 2 to your horizontal position, a total of 15.
//
// After following these instructions, you would have a horizontal position of 15 and a depth of 10. (Multiplying these together produces 150.)
//
// Calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?
//
// To begin, get your puzzle input.
// In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. The commands also mean something entirely different than you first thought:
//
//     down X increases your aim by X units.
//     up X decreases your aim by X units.
//     forward X does two things:
//         It increases your horizontal position by X units.
//         It increases your depth by your aim multiplied by X.
//
// Again note that since you're on a submarine, down and up do the opposite of what you might expect: "down" means aiming in the positive direction.
//
// Now, the above example does something different:
//
//     forward 5 adds 5 to your horizontal position, a total of 5. Because your aim is 0, your depth does not change.
//     down 5 adds 5 to your aim, resulting in a value of 5.
//     forward 8 adds 8 to your horizontal position, a total of 13. Because your aim is 5, your depth increases by 8*5=40.
//     up 3 decreases your aim by 3, resulting in a value of 2.
//     down 8 adds 8 to your aim, resulting in a value of 10.
//     forward 2 adds 2 to your horizontal position, a total of 15. Because your aim is 10, your depth increases by 2*10=20 to a total of 60.
//
// After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)
//
// Using this new interpretation of the commands, calculate the horizontal position and depth you would have after following the planned course. What do you get if you multiply your final horizontal position by your final depth?

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
