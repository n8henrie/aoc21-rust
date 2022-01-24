use std::{collections::HashMap, str::FromStr};

#[derive(Clone, PartialEq, Debug)]
enum Direction {
    Opening,
    Closing,
}

#[derive(Clone, PartialEq, Debug)]
enum Style {
    Round,
    Square,
    Curly,
    Angled,
}

#[derive(Clone, Debug)]
struct ChunkPoint((Direction, Style));

#[derive(Debug)]
struct Line(Vec<ChunkPoint>);

impl FromStr for Line {
    type Err = AocErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;
        use Style::*;

        Ok(Self(
            s.trim()
                .chars()
                .map(|c| match c {
                    '[' => Ok(ChunkPoint((Opening, Square))),
                    ']' => Ok(ChunkPoint((Closing, Square))),
                    '(' => Ok(ChunkPoint((Opening, Round))),
                    ')' => Ok(ChunkPoint((Closing, Round))),
                    '{' => Ok(ChunkPoint((Opening, Curly))),
                    '}' => Ok(ChunkPoint((Closing, Curly))),
                    '<' => Ok(ChunkPoint((Opening, Angled))),
                    '>' => Ok(ChunkPoint((Closing, Angled))),
                    c => Err(BadInputErr(c).into()),
                })
                .collect::<Result<_, Self::Err>>()?,
        ))
    }
}

#[derive(Debug)]
enum AocErr {
    Matching(MatchingErr),
    PrematureClosing(PrematureClosingErr),
    BadInput(BadInputErr),
}

#[derive(Debug)]
struct MatchingErr {
    expected: ChunkPoint,
    found: ChunkPoint,
}

#[derive(Debug)]
struct BadInputErr(char);

#[derive(Debug)]
struct PrematureClosingErr(ChunkPoint);

impl std::fmt::Display for ChunkPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(self))
    }
}

impl From<&ChunkPoint> for char {
    fn from(cp: &ChunkPoint) -> Self {
        use Direction::*;
        use Style::*;
        match cp {
            ChunkPoint((Opening, Square)) => '[',
            ChunkPoint((Closing, Square)) => ']',
            ChunkPoint((Opening, Round)) => '(',
            ChunkPoint((Closing, Round)) => ')',
            ChunkPoint((Opening, Curly)) => '{',
            ChunkPoint((Closing, Curly)) => '}',
            ChunkPoint((Opening, Angled)) => '<',
            ChunkPoint((Closing, Angled)) => '>',
        }
    }
}

impl std::fmt::Display for AocErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocErr::Matching(e) => {
                writeln!(f, "expected {}, found {}", e.expected, e.found)
            }
            AocErr::PrematureClosing(e) => {
                writeln!(f, "found unexpected {}", e.0)
            }
            AocErr::BadInput(e) => {
                writeln!(f, "bad input char {}", e.0)
            }
        }
    }
}
impl std::error::Error for AocErr {}

impl From<PrematureClosingErr> for AocErr {
    fn from(e: PrematureClosingErr) -> Self {
        Self::PrematureClosing(e)
    }
}

impl From<MatchingErr> for AocErr {
    fn from(e: MatchingErr) -> Self {
        Self::Matching(e)
    }
}

impl From<BadInputErr> for AocErr {
    fn from(e: BadInputErr) -> Self {
        Self::BadInput(e)
    }
}

impl Line {
    fn parse(&self) -> Result<Vec<ChunkPoint>, AocErr> {
        let mut stack = Vec::new();
        use Direction::*;
        for cp in self.0.iter() {
            match (stack.last(), cp) {
                // Happy path: opening is always ok
                (_, cp @ ChunkPoint((Opening, _))) => stack.push(cp.clone()),

                // Happy path: matched closer
                (
                    Some(&ChunkPoint((Opening, ref a))),
                    ChunkPoint((Closing, b)),
                ) if a == b => {
                    stack.pop();
                }

                // Stack empty but found closing
                (None, cp @ ChunkPoint((Closing, _))) => {
                    return Err(PrematureClosingErr(cp.clone()).into())
                }

                // Mismatch
                (Some(a), b) => {
                    let expected = if let ChunkPoint((Opening, s)) = a {
                        ChunkPoint((Closing, s.clone()))
                    } else {
                        unreachable!("only opening goes into the stack")
                    };
                    return Err(MatchingErr {
                        expected,
                        found: b.clone(),
                    }
                    .into());
                }
            }
        }
        Ok(stack)
    }

    fn get_completion(
        chunks: &[ChunkPoint],
    ) -> impl Iterator<Item = ChunkPoint> + '_ {
        chunks.iter().rev().map(|ChunkPoint((_, style))| {
            ChunkPoint((Direction::Closing, style.clone()))
        })
    }
}

fn points_map() -> HashMap<char, u32> {
    [(')', 3), (']', 57), ('}', 1197), ('>', 25137)].into()
}

fn points_map2() -> HashMap<char, u32> {
    [(')', 1), (']', 2), ('}', 3), ('>', 4)].into()
}

fn part1(input: &[Line]) -> Result<u32, AocErr> {
    let map = points_map();
    input
        .iter()
        .map(|line| {
            let val = match line.parse() {
                Err(e) => {
                    let c = char::from(&match e {
                        AocErr::Matching(e) => e.found,
                        AocErr::PrematureClosing(e) => e.0,
                        e => return Err(e),
                    });
                    map[&c]
                }
                _ => 0,
            };
            Ok(val)
        })
        .sum()
}

fn part2(input: &[Line]) -> u64 {
    let map = points_map2();
    let mut scores: Vec<_> = input
        .iter()
        .filter_map(|line| line.parse().ok())
        .map(|line| {
            Line::get_completion(&line).fold(0_u64, |acc, cp| {
                let c = char::from(&cp);
                let points = map[&c];
                acc * 5 + points as u64
            })
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn parse_input(input: &str) -> Result<Vec<Line>, AocErr> {
    input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<_, _>>()
}
fn main() -> Result<(), AocErr> {
    let input: Vec<Line> = parse_input(include_str!("../input.txt"))?;
    println!("day 10 part 1: {}", part1(&input)?);
    println!("day 10 part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        let input = parse_input(EXAMPLE_INPUT).unwrap();
        let expected = 26397;
        assert_eq!(part1(&input).unwrap(), expected);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(EXAMPLE_INPUT).unwrap();
        assert_eq!(part2(&input[..1]), 288957);

        let expected = 288957;
        assert_eq!(part2(&input), expected);
    }

    #[test]
    fn test_get_completion() {
        let expected = "}}]])})]
)}>]})
}}>}>))))
]]}}]}]}>
])}>";
        let completions: String = parse_input(EXAMPLE_INPUT)
            .unwrap()
            .iter()
            .filter_map(|line| {
                line.parse().ok().map(|line| {
                    Line::get_completion(&line)
                        .map(|cp| char::from(&cp))
                        .collect::<String>()
                })
            })
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(completions, expected);
    }
}
