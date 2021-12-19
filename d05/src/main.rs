use std::str::FromStr;

use aoc::localpath;

#[derive(Debug, PartialEq)]
struct Point((usize, usize));

#[derive(Debug, PartialEq)]
struct Line {
    start: Point,
    stop: Point,
}

macro_rules! delta {
    ($tt:ident, $idx:tt) => {{
        use ::std::cmp::Ordering::*;
        match ($tt.stop.0.$idx).cmp(&$tt.start.0.$idx) {
            Greater => 1,
            Equal => 0,
            Less => -1,
        }
    }};
}

impl Line {
    fn traverse(&self) -> impl Iterator<Item = Point> + '_ {
        let (x_delta, y_delta) = (delta!(self, 0), delta!(self, 1));
        let mut pos: (isize, isize) =
            (self.start.0 .0 as isize, self.start.0 .1 as isize);
        std::iter::once(Point(self.start.0)).chain(std::iter::from_fn(
            move || {
                if Point((pos.0 as usize, pos.1 as usize)) == self.stop {
                    None
                } else {
                    pos.0 += x_delta;
                    pos.1 += y_delta;
                    Some(Point((pos.0 as usize, pos.1 as usize)))
                }
            },
        ))
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitter = s.split(" -> ").map(|word| word.parse());
        match (splitter.next(), splitter.next(), splitter.next()) {
            (Some(Ok(first)), Some(Ok(second)), None) => Ok(Line {
                start: first,
                stop: second,
            }),
            _ => panic!("didn't work"),
        }
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(str::parse);
        if let (Some(Ok(x)), Some(Ok(y)), None) =
            (iter.next(), iter.next(), iter.next())
        {
            Ok(Self((x, y)))
        } else {
            anyhow::bail!("couldn't parse as Point: {}", s)
        }
    }
}

struct Diagram(Vec<Vec<u32>>);

impl Diagram {
    fn new(
        lines: Vec<Line>,
        dimensions: Point,
        include_diagonals: bool,
    ) -> Self {
        Self(lines.iter().fold(
            vec![vec![0; dimensions.0 .0]; dimensions.0 .1],
            |mut acc, line| {
                if !include_diagonals {
                    match (delta!(line, 0), delta!(line, 1)) {
                        (_, 0) | (0, _) => (),
                        _ => return acc,
                    };
                }
                for point in line.traverse() {
                    acc[point.0 .1][point.0 .0] += 1
                }
                acc
            },
        ))
    }

    fn iter(&self) -> impl Iterator<Item = &u32> {
        self.0.iter().flat_map(|row| row.iter())
    }
}

fn parse_input(
    input: &str,
    include_diagonals: bool,
) -> anyhow::Result<Diagram> {
    let (dimensions, lines) =
        input.lines().try_fold::<_, _, anyhow::Result<_>>(
            ((0, 0), Vec::new()),
            |(mut dim, mut lines), line| {
                let line: Line = line.parse()?;
                for point in [&line.start, &line.stop] {
                    // Account for input being zero indexed, so lengths need to
                    // be 1 larger
                    dim.0 = dim.0.max(point.0 .0 + 1);
                    dim.1 = dim.1.max(point.0 .1 + 1);
                }
                lines.push(line);
                Ok((dim, lines))
            },
        )?;
    Ok(Diagram::new(lines, Point(dimensions), include_diagonals))
}

fn solve(diagram: &Diagram) -> u32 {
    diagram.iter().filter(|&val| *val >= 2).count() as u32
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string(localpath!("input.txt"))?;
    let diagram1 = parse_input(&input, false)?;
    println!("day 05 part 1: {}", solve(&diagram1));
    let diagram2 = parse_input(&input, true)?;
    println!("day 05 part 2: {}", solve(&diagram2));
    Ok(())
}

#[cfg(test)]
mod tests;
