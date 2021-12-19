use super::*;

static EXAMPLE_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[test]
fn test_parse_point() {
    let point: Point = "0,9".parse().unwrap();
    let expected = Point((0, 9));
    assert_eq!(point, expected);
}

#[test]
fn test_parse_line() {
    let line: Line = EXAMPLE_INPUT.lines().next().unwrap().parse().unwrap();
    let expected = Line {
        start: Point((0, 9)),
        stop: Point((5, 9)),
    };
    assert_eq!(line, expected);
}

#[test]
fn test_part1() {
    let expected = 5;
    let diagram = parse_input(EXAMPLE_INPUT, false).unwrap();
    assert_eq!(solve(&diagram), expected);
}

#[test]
fn test_parse_input() {
    let diagram = parse_input(EXAMPLE_INPUT, false).unwrap();
    assert_eq!(diagram.0.len(), 10);
    assert_eq!(diagram.0[0].len(), 10);
    assert_eq!(diagram[(0, 9)], 2);
}

#[test]
fn test_traverse() {
    let line: Line = "3,4 -> 1,4".parse().unwrap();
    let traversed: Vec<Point> = line.traverse().collect();
    let expected = vec![Point((3, 4)), Point((2, 4)), Point((1, 4))];
    assert_eq!(traversed, expected);
    let line: Line = "6,2 -> 6,4".parse().unwrap();
    let traversed: Vec<Point> = line.traverse().collect();
    let expected = vec![Point((6, 2)), Point((6, 3)), Point((6, 4))];
    assert_eq!(traversed, expected);
}

#[test]
fn test_part2() {
    let expected = 12;
    let diagram = parse_input(EXAMPLE_INPUT, true).unwrap();
    assert_eq!(solve(&diagram), expected);
}
