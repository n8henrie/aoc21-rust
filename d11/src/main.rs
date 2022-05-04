#![warn(clippy::pedantic)]
use std::str::FromStr;
use std::{collections::HashSet, fmt};

enum Octopus {
    Flashed,
    Unflashed(u8),
}

struct Octopi(Vec<Vec<Octopus>>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl Octopi {
    fn enumerate_mut(
        &mut self,
    ) -> impl Iterator<Item = (Position, &mut Octopus)> {
        self.0.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, oct)| (Position { x, y }, oct))
        })
    }

    fn reset_flashed(&mut self) {
        for row in &mut self.0 {
            for oct in row {
                if let Octopus::Flashed = oct {
                    *oct = Octopus::Unflashed(0);
                }
            }
        }
    }

    fn neighbor_positions(
        target: &Position,
        (x_max, y_max): (usize, usize),
    ) -> impl Iterator<Item = Position> {
        const ADJ: [(usize, usize); 3] = [(0, 1), (1, 0), (1, 1)];
        let Position { x, y } = target;
        let hs: HashSet<Position> = ADJ
            .iter()
            .flat_map(move |(dx, dy)| {
                [
                    (x.checked_add(*dx), y.checked_add(*dy)),
                    (x.checked_sub(*dx), y.checked_sub(*dy)),
                    (x.checked_add(*dx), y.checked_sub(*dy)),
                    (x.checked_sub(*dx), y.checked_add(*dy)),
                ]
            })
            .filter_map(|idx| {
                if let (Some(x), Some(y)) = idx {
                    if x < x_max && y < y_max {
                        return Some(Position { x, y });
                    }
                }
                None
            })
            .collect();
        hs.into_iter()
    }

    fn flash_count(&mut self) -> u32 {
        let mut sum = 0;
        let (width, height) = (self.0[0].len(), self.0.len());

        // Initialize to all octopi in order to start by incrementing everything
        let mut neighbor_positions: Vec<_> = (0..(self.0.len()))
            .flat_map(|y| {
                (0..(self.0[0].len())).map(move |x| Position { x, y })
            })
            .collect();

        loop {
            let mut new_flashes = 0;

            // increment all unflashed neighbors
            while let Some(pos) = neighbor_positions.pop() {
                if let Some(Octopus::Unflashed(ref mut v)) =
                    self.0.get_mut(pos.y).and_then(|row| row.get_mut(pos.x))
                {
                    *v += 1;
                }
            }

            for (pos, oct) in self.enumerate_mut() {
                match oct {
                    Octopus::Unflashed(v) if *v >= 10 => {
                        new_flashes += 1;

                        *oct = Octopus::Flashed;
                        neighbor_positions.extend(Octopi::neighbor_positions(
                            &pos,
                            (width, height),
                        ));
                    }
                    _ => (),
                }
            }

            if new_flashes == 0 {
                break;
            }
            sum += new_flashes;
        }

        self.reset_flashed();
        sum
    }
}

impl fmt::Display for Octopi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|row| row
                    .iter()
                    .map(|oct| {
                        if let &Octopus::Unflashed(v) = oct {
                            v.to_string()
                        } else {
                            "F".to_string()
                        }
                    })
                    .collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl FromStr for Octopi {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Octopi(
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            d @ '0'..='9' => Ok(Octopus::Unflashed(
                                d.to_digit(10)
                                    .expect("couldn't make into digit")
                                    .try_into()
                                    .expect("couldn't make into u8"),
                            )),
                            _ => unreachable!("Bad input!"),
                        })
                        .collect()
                })
                .collect::<anyhow::Result<_>>()?,
        ))
    }
}

fn part1(octopi: &mut Octopi, days: usize) -> u32 {
    (0..days).map(|_| octopi.flash_count()).sum()
}

fn part2(octopi: &mut Octopi) -> u32 {
    let mut counter = 0;
    while !octopi.0.iter().flat_map(|row| row.iter()).all(|oct| {
        match (&octopi.0[0][0], oct) {
            (Octopus::Unflashed(first), Octopus::Unflashed(second))
                if first == second =>
            {
                true
            }
            _ => false,
        }
    }) {
        octopi.flash_count();
        counter += 1;
    }
    counter
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");
    let mut octopi: Octopi = input.parse()?;
    println!("day 11 part 1: {}", part1(&mut octopi, 100));
    println!("day 11 part 2: {}", part2(&mut octopi) + 100);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        let mut octopi: Octopi = EXAMPLE_INPUT.parse().unwrap();
        let expected = 1656;
        assert_eq!(part1(&mut octopi, 100), expected);
    }

    #[test]
    fn test_steps() {
        let small_example: &str = "11111
19991
19191
19991
11111";

        let mut octopi: Octopi = small_example.parse().unwrap();
        let steps = [
            "34543
40004
50005
40004
34543",
            "45654
51115
61116
51115
45654",
        ];

        for step in steps {
            let _ = octopi.flash_count();
            assert_eq!(octopi.to_string(), step);
        }
    }

    #[test]
    fn debug_steps() {
        let tiny_example: &str = "0000
0800
9000";

        let mut octopi: Octopi = tiny_example.parse().unwrap();
        let steps = [
            "2221
3021
0321",
            "3332
4132
1432",
            "4443
5243
2543",
        ];

        for expected in steps {
            let _ = octopi.flash_count();
            println!("{}\n\n{}", octopi, expected);
            assert_eq!(octopi.to_string(), expected);
        }
    }

    #[test]
    fn test_neighbor_positions() {
        let tests = [
            (Position { x: 3, y: 9 }, (20, 20)),
            (Position { x: 0, y: 0 }, (5, 5)),
            (Position { x: 5, y: 10 }, (6, 11)),
            (Position { x: 5, y: 5 }, (6, 11)),
        ];
        let mut expected = [
            vec![
                (2_usize, 9_usize),
                (4, 9),
                (3, 8),
                (3, 10),
                (2, 8),
                (4, 10),
                (4, 8),
                (2, 10),
            ],
            vec![(0, 1), (1, 0), (1, 1)],
            vec![(5, 9), (4, 10), (4, 9)],
            vec![(5, 4), (5, 6), (4, 5), (4, 4), (4, 6)],
        ];

        assert_eq!(tests.len(), expected.len());

        for ((start, (xmax, ymax)), output) in
            tests.iter().zip(expected.iter_mut())
        {
            let mut pos: Vec<_> =
                Octopi::neighbor_positions(start, (*xmax, *ymax))
                    .map(|p| (p.x, p.y))
                    .collect();
            pos.sort_unstable();
            output.sort_unstable();

            assert_eq!(&pos, output);
        }
    }

    #[test]
    fn test_fmt() {
        let octopi: Octopi = EXAMPLE_INPUT.parse().unwrap();
        let output = octopi.to_string();
        assert_eq!(output, EXAMPLE_INPUT);
    }

    #[test]
    fn test_part2() {
        let mut octopi: Octopi = EXAMPLE_INPUT.parse().unwrap();
        let expected = 195;
        assert_eq!(part2(&mut octopi), expected);
    }
}
