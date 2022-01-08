use aoc::{localpath, parse_input};
use std::collections::{BTreeSet, HashMap, HashSet};

fn part1(input: &[String]) -> usize {
    let map = base_known_map();
    input
        .iter()
        .flat_map(|line| {
            let mut iter =
                line.split_whitespace().skip_while(|word| word != &"|");
            let _bar = iter.next();
            iter
        })
        .filter(|word| {
            let len = word.chars().count();
            matches!(map.get(&(len as u8)), Some(&Display::Solved(_)))
        })
        .count()
}

#[derive(Debug)]
struct AnswerKey(HashMap<BTreeSet<char>, Display>);

impl AnswerKey {
    fn new<'a>(
        words: impl Iterator<Item = &'a BTreeSet<char>>,
    ) -> anyhow::Result<Self> {
        let mut hm = HashMap::new();

        let defaults = base_known_map();
        for word in words {
            if let Some(val) = defaults.get(&(word.len() as u8)) {
                hm.insert(word.clone(), val.clone());
            } else {
                anyhow::bail!("no default for length {}", word.len());
            }
        }
        Ok(Self(hm))
    }

    fn get<T>(&self, key: T) -> Option<&Display>
    where
        T: Iterator<Item = char>,
    {
        let key: BTreeSet<char> = key.collect();
        self.0.get(&key)
    }

    fn iter(&self) -> impl Iterator<Item = (&BTreeSet<char>, &Display)> {
        self.0.iter()
    }

    fn find_word_for(&self, val: u8) -> Option<BTreeSet<char>> {
        self.iter()
            .find(|(_, v)| **v == Display::Solved(val))
            .map(|(k, _)| k.clone())
    }

    fn solve(&mut self) {
        // Should be solved by default in `base_known_map`
        let one = self
            .find_word_for(1)
            .expect("unreachable: one should already be solved");
        let four = self
            .find_word_for(4)
            .expect("unreachable: four should already be solved");
        let eight = self
            .find_word_for(8)
            .expect("unreachable: eight should already be solved");

        for (word, disp) in self.0.iter_mut() {
            match disp {
                Display::Solved(_) => continue,
                Display::Unsolved => {
                    *disp = match word.len() {
                        // 2, 3, and 5 have 5 segments
                        5 => {
                            match (
                                word.intersection(&one).count(),
                                word.intersection(&four).count(),
                            ) {
                                (2, _) => Display::Solved(3),
                                (1, 2) => Display::Solved(2),
                                (1, 3) => Display::Solved(5),
                                _ => unreachable!(
                                    "3 should be solved, leaving only 2 and 5"
                                ),
                            }
                        }

                        // 0, 6, and 9 have 6 segments
                        6 => {
                            let empty_segment = eight
                                .difference(word)
                                .next()
                                .expect(
                                "unreachable: already matched on 6 segments",
                            );

                            match (
                                word.intersection(&one).count(),
                                four.contains(empty_segment),
                            ) {
                                (1, _) => {
                                    // 0 and 9 both share 2 segments with 1
                                    Display::Solved(6)
                                }
                                (2, true) => Display::Solved(0),
                                (2, false) => Display::Solved(9),
                                _ => unreachable!("logic error"),
                            }
                        }
                        _ => unreachable!("all digits should be solved"),
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Display {
    Unsolved,
    Solved(u8),
}

fn decode_numbers(input: &str) -> anyhow::Result<AnswerKey> {
    let mut iter = input.split_whitespace();
    let first_half: HashSet<BTreeSet<char>> = iter
        .by_ref()
        .take_while(|word| word != &"|")
        .map(|word| BTreeSet::from_iter(word.chars()))
        .collect();
    let output_values: HashSet<BTreeSet<char>> =
        iter.map(|word| BTreeSet::from_iter(word.chars())).collect();
    let mut ak =
        AnswerKey::new(first_half.iter().chain(output_values.iter()))?;

    ak.solve();
    Ok(ak)
}
fn part2(input: &[String]) -> anyhow::Result<u32> {
    input
        .iter()
        .map(|line| {
            let ak = decode_numbers(line)?;
            let mut iter = line.split_whitespace().skip_while(|v| v != &"|");
            let _separator = iter.next();
            let coded: Vec<_> = iter.collect();
            coded
                .iter()
                .map(|code| {
                    if let Some(Display::Solved(v)) = ak.get(code.chars()) {
                        Ok(v.to_string())
                    } else {
                        anyhow::bail!("unknown code: {}", code)
                    }
                })
                .collect::<anyhow::Result<String>>()?
                .parse::<u32>()
                .map_err(|e| anyhow::anyhow!(e))
        })
        .sum::<Result<u32, _>>()
}

/// Generate defaults based on the number of segments in a "word". Some can be
/// solved based on this information alone (see `part1`) whereas others can be
/// narrowed down. The resulting map is {num_segments: possible_numbers}
fn base_known_map() -> HashMap<u8, Display> {
    let mut hm = <HashMap<u8, Display>>::new();
    for (num_segments, val) in [(7, 8), (2, 1), (3, 7), (4, 4)] {
        hm.insert(num_segments, Display::Solved(val));
    }
    for (num_segments, val) in [(6, Display::Unsolved), (5, Display::Unsolved)]
    {
        hm.insert(num_segments, val);
    }
    hm
}

fn main() -> anyhow::Result<()> {
    let input = parse_input!(localpath!("input.txt"))?;
    println!("day 08 part 1: {}", part1(&input));
    println!("day 08 part 2: {}", part2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input!(EXAMPLE_INPUT).unwrap()), 26);
    }
    #[test]
    fn test_part2() {
        let result = part2(&parse_input!(EXAMPLE_INPUT).unwrap()).unwrap();
        let expected = 61229;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_decode_numbers() {
        let lines = parse_input!(EXAMPLE_INPUT).unwrap();
        let ak = decode_numbers(lines.first().unwrap()).unwrap();
        assert_eq!(ak.get("be".chars()), Some(&Display::Solved(1)));
        assert_eq!(ak.get("edb".chars()), Some(&Display::Solved(7)));
    }
}
