use std::collections::HashMap;
use nom::{IResult, branch::alt, bytes::complete::tag, multi::many1, character, Parser};

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day22/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 67390);
}

fn part1(input: &str) -> i32 {
    // Parse the input
    let (board_str, moves) = input.split_once("\n\n").unwrap();
    let moves = moves.trim_end();
    let mut board = HashMap::<(i32, i32), Board>::new();
    for (row,line) in board_str.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                '.' => { board.insert((row as i32, col as i32), Board::Open); },
                '#' => { board.insert((row as i32, col as i32), Board::Wall); },
                _ => {}
            };
        }
    }

    // Split `moves` into runs of digits or single letter
    let moves = parse_moves(moves).unwrap().1;

    // Find the starting position (the first Open in row #0)
    let mut facing = Facing::Right;
    let mut row = 0;
    let mut col = 0;
    while board.get(&(row, col)) != Some(&Board::Open) {
        col += 1;
    }

    // Follow the moves, with possible wrap-around
    for m in moves {
        match m {
            Move::Left => {
                facing = facing.turn_left();
            },
            Move::Right => {
                facing = facing.turn_right();
            },
            Move::Number(num) => {
                for _ in 0 .. num {
                    // Try to move one space in the current direction
                    (row, col) = board_try_move(&board, row, col, facing);
                }
            },
        }
    }

    // Compute the "password"
    1000 * (row + 1) + 4 * (col + 1) + (facing as i32)
}

fn board_try_move(
    board: &HashMap<(i32, i32), Board>,
    row: i32, col: i32, facing: Facing)
    -> (i32, i32)
{
    let (d_row, d_col) = match facing {
        Facing::Right => (0, 1),
        Facing::Left => (0, -1),
        Facing::Up => (-1, 0),
        Facing::Down => (1, 0),
    };

    // If the new position is open, move there
    let result = board.get(&(row+d_row, col+d_col));
    match result {
        Some(&Board::Open) => { return (row+d_row, col+d_col); },
        Some(&Board::Wall) => { return (row, col); },
        None => {}
    }

    // Wrap around and return the first Some()
    let mut r = row;
    let mut c = col;
    match facing {
        Facing::Right => { c = 0; },
        Facing::Left => {
            // Go to largest possible column in row
            c = *board.keys().filter(|(rr,_)| *rr==row).map(|(_,cc)| cc).max().unwrap();
        },
        Facing::Down => { r = 0; },
        Facing::Up => {
            // Go to largest possible row in column
            r = *board.keys().filter(|(_,cc)| *cc==col).map(|(rr,_)| rr).max().unwrap();
        },
    }
    
    let mut result = board.get(&(r, c));
    while result.is_none() {
        r += d_row;
        c += d_col;
        result = board.get(&(r, c));
    }
    
    if result == Some(&Board::Open) {
        (r,c)
    } else {
        (row, col)
    }
}

fn parse_moves(s: &str) -> IResult<&str,Vec<Move>> {
    many1(alt((
        tag("L").map(|_| Move::Left),
        tag("R").map(|_| Move::Right),
        character::complete::i32.map(|num| Move::Number(num))
    )))
    .parse(s)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Board {
    Open,
    Wall
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3
}

impl Facing {
    fn turn_right(&self) -> Self {
        use Facing::*;
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right
        }
    }

    fn turn_left(&self) -> Self {
        use Facing::*;
        match self {
            Right => Up,
            Up => Left,
            Left => Down,
            Down => Right
        }
    }
}

#[derive(Debug)]
enum Move {
    Number(i32),
    Left,
    Right
}

#[test]
fn test_part1() {
    let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";
    assert_eq!(part1(input), 6032);
}