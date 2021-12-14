use aoc::{localpath, parse_input};
use color_eyre::eyre::{self, WrapErr};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
struct ReportNumber<const N: usize>([bool; N]);

impl<const N: usize> std::str::FromStr for ReportNumber<N> {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self, Self::Err> {
        let vec: Vec<_> = s
            .chars()
            .map(|c| match c {
                '0' => Ok(false),
                '1' => Ok(true),
                _ => eyre::bail!("should only be 1s and 0s in input"),
            })
            .collect::<eyre::Result<_>>()?;
        let array = vec.try_into().map_err(|v: Vec<_>| {
            eyre::eyre!("Unable to create report from vec size {}", v.len())
        })?;
        Ok(ReportNumber(array))
    }
}

struct Report<const N: usize>(Vec<ReportNumber<N>>);

impl<T, const N: usize> From<T> for Report<N>
where
    T: IntoIterator<Item = ReportNumber<N>>,
{
    fn from(t: T) -> Self {
        Self(t.into_iter().collect())
    }
}

impl<const N: usize> Report<N> {
    fn iter(&self) -> impl Iterator<Item = [bool; N]> + '_ {
        self.0.iter().map(|report| report.0)
    }
}

impl<const N: usize> AsRef<[bool]> for ReportNumber<N> {
    fn as_ref(&self) -> &[bool] {
        &self.0
    }
}

impl<const N: usize> AsRef<[ReportNumber<N>]> for Report<N> {
    fn as_ref(&self) -> &[ReportNumber<N>] {
        &self.0
    }
}

fn part1<const N: usize>(report: &Report<N>) -> eyre::Result<u32> {
    let results = report.iter().fold([0_f32; N], |acc, arr| {
        acc.iter()
            .zip(arr.iter())
            .map(|(a, b)| {
                a + match b {
                    false => 0_f32,
                    true => 1_f32,
                }
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("wrong size vec")
    });
    let count = report.0.len();
    let gamma: String = results
        .iter()
        .map(|&v| match (v / count as f32).round() == 1.0 {
            true => '1',
            false => '0',
        })
        .collect();
    let epsilon: String = gamma
        .chars()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            _ => unreachable!("only 1s and zeros set in gamma"),
        })
        .collect();
    let gamma = u32::from_str_radix(&gamma, 2)?;
    let epsilon = u32::from_str_radix(&epsilon, 2)?;
    Ok(gamma * epsilon)
}

fn from_binary(input: impl AsRef<[bool]>) -> eyre::Result<u32> {
    let s: String = input
        .as_ref()
        .iter()
        .map(|v| match v {
            true => '1',
            false => '0',
        })
        .collect();
    u32::from_str_radix(&s, 2).wrap_err("unable to parse as binary: {}, s")
}

fn filter_rows<const N: usize>(
    rows: &[ReportNumber<N>],
    take_greater: bool,
) -> usize {
    let mut col_idx = 0;
    let mut filter = <HashSet<usize>>::new();
    loop {
        let filtered: Vec<(usize, &[bool])> = rows
            .iter()
            .enumerate()
            .filter_map(|(idx, row)| {
                if !filter.contains(&idx) {
                    Some((idx, row.as_ref()))
                } else {
                    None
                }
            })
            .collect();
        if filtered.len() == 1 {
            break filtered[0].0;
        }
        let (ones, zeros): (Vec<_>, Vec<_>) =
            filtered.iter().partition(|(_, val)| val[col_idx]);

        let add_to_filter = match ones.len().cmp(&zeros.len()) {
            std::cmp::Ordering::Less => {
                if take_greater {
                    ones
                } else {
                    zeros
                }
            }
            std::cmp::Ordering::Equal => {
                if take_greater {
                    zeros
                } else {
                    ones
                }
            }
            std::cmp::Ordering::Greater => {
                if take_greater {
                    zeros
                } else {
                    ones
                }
            }
        };
        filter.extend(add_to_filter.iter().map(|(idx, _)| *idx));
        col_idx += 1;
    }
}

fn get_o2_rating<const N: usize>(report: &Report<N>) -> eyre::Result<u32> {
    let idx = filter_rows(report.as_ref(), true);
    from_binary(report.0[idx].as_ref())
}

fn get_co2_rating<const N: usize>(report: &Report<N>) -> eyre::Result<u32> {
    let idx = filter_rows(report.as_ref(), false);
    from_binary(report.0[idx].as_ref())
}

fn part2<const N: usize>(report: &Report<N>) -> eyre::Result<u32> {
    let o2 = get_o2_rating(report)?;
    let co2 = get_co2_rating(report)?;
    Ok(o2 * co2)
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let report: Report<12> =
        parse_input!(localpath!("input.txt"), ReportNumber<12>)
            .map_err(|e| eyre::eyre!(e))?
            .into();
    println!("day 03 part 1: {}", part1(&report).unwrap());
    println!("day 03 part 2: {}", part2(&report).unwrap());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_parse() {
        let parsed: Report<5> =
            parse_input!(TEST_INPUT, ReportNumber<5>).unwrap().into();
        assert_eq!(parsed.0[0].as_ref(), [false, false, true, false, false]);
    }

    #[test]
    fn test_part1() {
        let report = parse_input!(TEST_INPUT, ReportNumber<5>).unwrap().into();
        let expected = 198;
        assert_eq!(part1(&report).unwrap(), expected);
    }

    #[test]
    fn test_part2() {
        let report = parse_input!(TEST_INPUT, ReportNumber<5>).unwrap().into();
        let expected = 230;
        assert_eq!(part2(&report).unwrap(), expected);
    }

    #[test]
    fn test_o2() {
        let report = parse_input!(TEST_INPUT, ReportNumber<5>).unwrap().into();
        let expected = 23;
        assert_eq!(get_o2_rating(&report).unwrap(), expected);
    }

    #[test]
    fn test_co2() {
        let report = parse_input!(TEST_INPUT, ReportNumber<5>).unwrap().into();
        let expected = 10;
        assert_eq!(get_co2_rating(&report).unwrap(), expected);
    }
}
