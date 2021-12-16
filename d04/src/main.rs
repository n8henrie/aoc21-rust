use anyhow::{anyhow, Context};
use aoc::localpath;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
enum Number {
    Marked(u32),
    Unmarked(u32),
}

#[derive(PartialEq, Debug, Clone)]
struct BingoBoard([[Number; 5]; 5]);

#[derive(Clone)]
struct BingoGame {
    numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl FromStr for BingoBoard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<_> = s
            .lines()
            .map(|line| {
                let row: Vec<_> = line
                    .split_whitespace()
                    .map(|word| {
                        Ok(Number::Unmarked(
                            word.parse().context("couldn't parse number")?,
                        ))
                    })
                    .collect::<anyhow::Result<_>>()?;
                row.try_into().map_err(|row| {
                    anyhow::anyhow!("couldn't make array from row: {:?}", row)
                })
            })
            .collect::<anyhow::Result<_>>()?;
        Ok(Self(vec.try_into().map_err(|row| {
            anyhow::anyhow!("couldn't convert rows to [row]: {:?}", row)
        })?))
    }
}

impl FromStr for BingoGame {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let numbers = lines
            .get(0)
            .context("no first line")?
            .split(',')
            .map(|word| word.parse::<u32>().map_err(|e| anyhow!(e)))
            .collect::<anyhow::Result<_>>()?;
        let mut boards = Vec::new();
        for chunk in lines[1..].chunks(6) {
            // Skip leading blank line
            let board: BingoBoard = chunk[1..].join("\n").parse()?;
            boards.push(board)
        }
        Ok(BingoGame { numbers, boards })
    }
}

fn parse_game(input: &str) -> anyhow::Result<BingoGame> {
    std::fs::read_to_string(localpath!(input))?.parse()
}

impl BingoBoard {
    fn is_winner(&self) -> bool {
        self.0
            .iter()
            .any(|row| row.iter().all(|num| matches!(num, Number::Marked(_))))
            || (0..self.0[0].len()).any(|col_idx| {
                self.0
                    .iter()
                    .all(|row| matches!(row[col_idx], Number::Marked(_)))
            })
    }

    fn play(&mut self, number: u32) {
        for row in self.0.iter_mut() {
            for num in row.iter_mut() {
                match num {
                    Number::Unmarked(n) if *n == number => {
                        *num = Number::Marked(number)
                    }
                    _ => (),
                }
            }
        }
    }

    fn score(&self, winning_number: u32) -> u32 {
        let points = self
            .0
            .iter()
            .flatten()
            .map(|num| match num {
                Number::Unmarked(val) => *val,
                _ => 0,
            })
            .sum::<u32>();
        points * winning_number
    }
}

impl BingoGame {
    fn play(&mut self, number: u32) {
        for board in self.boards.iter_mut() {
            board.play(number)
        }
    }
}

fn part1(game: &mut BingoGame) -> Option<u32> {
    let numbers = game.numbers.clone();
    for number in numbers {
        game.play(number);
        for board in game.boards.iter() {
            if board.is_winner() {
                return Some(board.score(number));
            }
        }
    }
    None
}

fn part2(game: &mut BingoGame) -> Option<u32> {
    let numbers = game.numbers.clone();
    for number in numbers {
        game.play(number);
        if game.boards.len() == 1 {
            let last_board = game.boards.first()?;
            if last_board.is_winner() {
                return Some(last_board.score(number));
            }
        }
        game.boards.retain(|board| !board.is_winner());
    }
    None
}

fn main() -> anyhow::Result<()> {
    let mut game1 = parse_game("input.txt")?;
    let mut game2 = game1.clone();
    println!(
        "day 04 part 1: {}",
        part1(&mut game1).ok_or_else(|| anyhow!("No winner for part 1"))?
    );
    println!(
        "day 04 part 2: {}",
        part2(&mut game2).ok_or_else(|| anyhow!("No winner for part 2"))?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn test_parse_board() {
        let board: BingoBoard = "22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19"
            .parse()
            .unwrap();
        let expected = BingoBoard([
            [
                Number::Unmarked(22),
                Number::Unmarked(13),
                Number::Unmarked(17),
                Number::Unmarked(11),
                Number::Unmarked(0),
            ],
            [
                Number::Unmarked(8),
                Number::Unmarked(2),
                Number::Unmarked(23),
                Number::Unmarked(4),
                Number::Unmarked(24),
            ],
            [
                Number::Unmarked(21),
                Number::Unmarked(9),
                Number::Unmarked(14),
                Number::Unmarked(16),
                Number::Unmarked(7),
            ],
            [
                Number::Unmarked(6),
                Number::Unmarked(10),
                Number::Unmarked(3),
                Number::Unmarked(18),
                Number::Unmarked(5),
            ],
            [
                Number::Unmarked(1),
                Number::Unmarked(12),
                Number::Unmarked(20),
                Number::Unmarked(15),
                Number::Unmarked(19),
            ],
        ]);
        assert_eq!(board, expected);
    }

    #[test]
    fn test_parse_game() {
        let game: BingoGame = parse_game("input.txt").unwrap();
        assert!(game.numbers.first().unwrap() == &99);
        assert!(game.numbers.last().unwrap() == &39);
        assert!(
            game.boards
                .first()
                .unwrap()
                .0
                .first()
                .unwrap()
                .first()
                .unwrap()
                == &Number::Unmarked(57)
        );
        assert!(
            game.boards
                .last()
                .unwrap()
                .0
                .last()
                .unwrap()
                .last()
                .unwrap()
                == &Number::Unmarked(47)
        );
    }

    #[test]
    fn test_part1() {
        let mut game: BingoGame = TEST_INPUT.parse().unwrap();
        assert_eq!(part1(&mut game).unwrap(), 4512);
    }

    #[test]
    fn test_part2() {
        let mut game: BingoGame = TEST_INPUT.parse().unwrap();
        assert_eq!(part2(&mut game).unwrap(), 1924);
    }

    #[test]
    fn test_winner() {
        let base = BingoBoard([
            [
                Number::Marked(22),
                Number::Unmarked(13),
                Number::Unmarked(17),
                Number::Unmarked(11),
                Number::Unmarked(0),
            ],
            [
                Number::Unmarked(8),
                Number::Marked(2),
                Number::Marked(23),
                Number::Marked(4),
                Number::Marked(24),
            ],
            [
                Number::Unmarked(21),
                Number::Marked(9),
                Number::Unmarked(14),
                Number::Unmarked(16),
                Number::Unmarked(7),
            ],
            [
                Number::Unmarked(6),
                Number::Marked(10),
                Number::Unmarked(3),
                Number::Unmarked(18),
                Number::Unmarked(5),
            ],
            [
                Number::Unmarked(1),
                Number::Marked(12),
                Number::Unmarked(20),
                Number::Unmarked(15),
                Number::Unmarked(19),
            ],
        ]);
        let mut col_winner = base.clone();
        *col_winner.0.get_mut(0).unwrap().get_mut(1).unwrap() =
            Number::Marked(2);
        assert!(col_winner.is_winner());

        let mut row_winner = base.clone();
        *row_winner.0.get_mut(1).unwrap().get_mut(0).unwrap() =
            Number::Marked(2);
        assert!(row_winner.is_winner());

        assert!(!base.is_winner());
    }
}
