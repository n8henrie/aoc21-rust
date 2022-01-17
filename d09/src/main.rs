use std::{collections::HashSet, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
enum AocError {
    #[error("Can't parse input as digit")]
    BadDigit(char),
    #[error("Can't make into u8")]
    BadU8 { source: std::num::TryFromIntError },
    #[error("Can't convert into [T; {arr_length}, found length {length}")]
    BadArray { length: usize, arr_length: usize },
    #[error("Not enough basins found")]
    NotEnoughBasins,
}

fn part1<const T: usize, const U: usize>(map: &Map<T, U>) -> u32 {
    map.low_points().map(|(_, val)| val as u32 + 1).sum()
}

fn part2<const T: usize, const U: usize>(
    map: &Map<T, U>,
) -> Result<u32, AocError> {
    let mut basins_by_size: Vec<_> = map.basins().map(|b| b.len()).collect();
    basins_by_size.sort_unstable();
    basins_by_size.reverse();
    basins_by_size
        .iter()
        .take(3)
        .map(|v| *v as u32)
        .reduce(std::ops::Mul::mul)
        .ok_or(AocError::NotEnoughBasins)
}

fn main() -> Result<(), AocError> {
    let map: Map<100, 100> = include_str!("../input.txt").parse()?;
    println!("day 09 part 1: {}", part1(&map));
    println!("day 09 part 2: {}", part2(&map)?);
    Ok(())
}

struct Map<const T: usize, const U: usize>([[u8; T]; U]);

impl<const T: usize, const U: usize> Map<T, U> {
    /// get the value at index (x, y)
    fn get(&self, index: (usize, usize)) -> Option<u8> {
        self.0
            .get(index.1)
            .and_then(|row| row.get(index.0).copied())
    }

    fn neighbors(
        &self,
        index: (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
        let deltas = [(0, 1), (1, 0)];
        let ops = [usize::checked_sub, usize::checked_add];
        ops.into_iter().flat_map(move |op| {
            deltas
                .iter()
                .filter_map(|(x, y)| {
                    match (op(index.0, *x), op(index.1, *y)) {
                        (Some(x), Some(y)) => {
                            self.get((x, y)).map(|v| ((x, y), v))
                        }
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
        })
    }

    fn items(&self) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
        self.0.iter().enumerate().flat_map(|(y_idx, row)| {
            row.iter()
                .enumerate()
                .map(move |(x_idx, val)| ((x_idx, y_idx), *val))
        })
    }

    fn low_points(&self) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
        self.items().filter_map(|(idx, val)| {
            let min_neighbor = self
                .neighbors(idx)
                .map(|(_, v)| v)
                .min()
                .expect("unreachable: indices have been checked");
            if val < min_neighbor {
                Some((idx, val))
            } else {
                None
            }
        })
    }

    fn basins(&self) -> impl Iterator<Item = HashSet<(usize, usize)>> + '_ {
        fn recurse<const T: usize, const U: usize>(
            map: &Map<T, U>,
            idx: (usize, usize),
            basin: &mut HashSet<(usize, usize)>,
        ) {
            if !basin.insert(idx) {
                return;
            }
            for (idx, val) in map.neighbors(idx) {
                if val != 9 {
                    recurse(map, idx, basin);
                }
            }
        }
        self.low_points().map(|(idx, _)| {
            let mut basin = HashSet::new();
            recurse(self, idx, &mut basin);
            basin
        })
    }
}

impl<const T: usize, const U: usize> FromStr for Map<T, U> {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    line.chars()
                        .map(|c| {
                            TryInto::<u8>::try_into(
                                c.to_digit(10).ok_or(AocError::BadDigit(c))?,
                            )
                            .map_err(|source| AocError::BadU8 { source })
                        })
                        .collect::<Result<Vec<_>, _>>()?
                        .try_into()
                        .map_err(|v: Vec<_>| AocError::BadArray {
                            arr_length: T,
                            length: v.len(),
                        })
                })
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|v: Vec<_>| AocError::BadArray {
                    length: v.len(),
                    arr_length: U,
                })?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";
    type TestMap = Map<10, 5>;

    #[test]
    fn test_parse() {
        let map: TestMap = EXAMPLE_INPUT.parse().unwrap();
        assert_eq!(map.get((0, 0)).unwrap(), 2);
        assert_eq!(map.get((9, 0)).unwrap(), 0);
        assert_eq!(map.get((0, 4)).unwrap(), 9);
        assert_eq!(map.get((9, 4)).unwrap(), 8);
        assert_eq!(map.get((0, 5)), None);
        assert_eq!(map.get((10, 0)), None);
    }

    #[test]
    fn test_part1() {
        let map: TestMap = EXAMPLE_INPUT.parse().unwrap();
        let result = part1(&map);
        let expected = 15;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_neighbors() {
        let map: TestMap = EXAMPLE_INPUT.parse().unwrap();
        let neighbors: Vec<_> =
            map.neighbors((1, 1)).map(|(_, v)| v).collect();
        let expected = vec![1, 3, 8, 8];
        assert_eq!(neighbors, expected);
        let neighbors: Vec<_> =
            map.neighbors((9, 4)).map(|(_, v)| v).collect();
        let expected = vec![9, 7];
        assert_eq!(neighbors, expected);
    }

    #[test]
    fn test_items() {
        let map: TestMap = EXAMPLE_INPUT.parse().unwrap();
        let indices: Vec<_> = map.items().collect();
        assert_eq!(indices[0], ((0, 0), 2));
        assert_eq!(indices.last(), Some(&((9, 4), 8)));
    }

    #[test]
    fn test_part2() {
        let map: TestMap = EXAMPLE_INPUT.parse().unwrap();
        let expected = 1134;
        assert_eq!(part2(&map).unwrap(), expected)
    }
}
