use std::collections::HashSet;
use std::ops::{Add, AddAssign};
use pathfinding::prelude::astar;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day24/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 299);

    let result2 = part2(&input);
    println!("Part 2: {result2}");
    // assert_eq!(result2, 299);
}

#[test]
fn test_part1() {
    let input = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
    assert_eq!(part1(input), 18);
}

#[test]
fn test_part2() {
    let input = "\
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
    assert_eq!(part2(input), 54);
}

fn part1(input: &str) -> i32 {
    let (blizzards, walls, width, height) = parse_input(&input);
    let start = State{ position: Point::new(1,0), blizzards };
    let end = Point::new(width-2, height-1);
    let success = |state: &State| state.position == end;
    let heuristic = |state: &State| -> i32 {
        // Minimum cost: Manhattan distance to the end point
        state.position.dist(&end)
    };
    let successors = |state: &State| -> Vec<(State, i32)> {
        if state.position == end {
            return Vec::new();
        }

        // Compute new locations of the blizzards
        let mut moved_blizzards = Vec::new();
        for &(Point{mut x, mut y}, dir) in state.blizzards.iter() {
            match dir {
                Direction::Up => {
                    y -= 1;
                    if y == 0 {
                        y = height - 2;
                    }
                },
                Direction::Down => {
                    y += 1;
                    if y == height - 1 {
                        y = 1;
                    }
                },
                Direction::Left => {
                    x -= 1;
                    if x == 0 {
                        x = width - 2;
                    }
                },
                Direction::Right => {
                    x += 1;
                    if x == width - 1 {
                        x = 1;
                    }
                },
            }
            moved_blizzards.push((Point::new(x, y), dir));
        }

        // Make a set of blizzard positions
        let blizzard_positions = moved_blizzards.iter()
            .map(|&(point,_dir)| point)
            .collect::<HashSet<_>>();
        
        // Figure out which positions we could move into that won't contain
        // a blizzard.
        let mut result = Vec::new();
        for d_point in [Point::new(1, 0), Point::new(0, 1), Point::new(0, 0), Point::new(-1, 0), Point::new(0, -1)] {
            let position = state.position + d_point;
            if !walls.contains(&position) && !blizzard_positions.contains(&position) {
                result.push((State{position, blizzards: moved_blizzards.clone()}, 1));
            }
        }
        result
    };
    let (_, steps) = astar(&start, successors, heuristic, success).unwrap();
    steps
}

fn part2(input: &str) -> i32 {
    let (blizzards, walls, width, height) = parse_input(&input);
    let start = Point::new(1,0);
    let end = Point::new(width-2, height-1);
    let success_end = |state: &State| state.position == end;
    let success_start = |state: &State| state.position == start;
    let heuristic_end = |state: &State| -> i32 {
        // Minimum cost: Manhattan distance to the end point
        state.position.dist(&end)
    };
    let heuristic_start = |state: &State| -> i32 {
        // Minimum cost: Manhattan distance to the starting point
        state.position.dist(&start)
    };
    let successors = |state: &State| -> Vec<(State, i32)> {
        // Compute new locations of the blizzards
        let mut moved_blizzards = Vec::new();
        for &(Point{mut x, mut y}, dir) in state.blizzards.iter() {
            match dir {
                Direction::Up => {
                    y -= 1;
                    if y == 0 {
                        y = height - 2;
                    }
                },
                Direction::Down => {
                    y += 1;
                    if y == height - 1 {
                        y = 1;
                    }
                },
                Direction::Left => {
                    x -= 1;
                    if x == 0 {
                        x = width - 2;
                    }
                },
                Direction::Right => {
                    x += 1;
                    if x == width - 1 {
                        x = 1;
                    }
                },
            }
            moved_blizzards.push((Point::new(x, y), dir));
        }

        // Make a set of blizzard positions
        let blizzard_positions = moved_blizzards.iter()
            .map(|&(point,_dir)| point)
            .collect::<HashSet<_>>();
        
        // Figure out which positions we could move into that won't contain
        // a blizzard.
        let mut result = Vec::new();
        for d_point in [Point::new(1, 0), Point::new(0, 1), Point::new(0, 0), Point::new(-1, 0), Point::new(0, -1)] {
            let position = state.position + d_point;
            if !walls.contains(&position) && !blizzard_positions.contains(&position) {
                result.push((State{position, blizzards: moved_blizzards.clone()}, 1));
            }
        }
        result
    };

    let mut total_steps = 0;

    // Go from start to end
    let initial_state = State{position: start, blizzards};
    let (path, steps) = astar(&initial_state, successors, heuristic_end, success_end).unwrap();
    total_steps += dbg!(steps);

    // Go from end to start
    let (path, steps) = astar(path.last().unwrap(), successors, heuristic_start, success_start).unwrap();
    total_steps += dbg!(steps);

    // Go from start to end
    let (_path, steps) = astar(path.last().unwrap(), successors, heuristic_end, success_end).unwrap();
    total_steps += dbg!(steps);

    total_steps
}

fn parse_input(input: &str) -> (Vec<(Point, Direction)>, HashSet<Point>, i32, i32) {
    let mut width = 0;
    let mut height = 0;
    let mut blizzards = Vec::new();
    let mut walls = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        width = line.len() as i32;
        height += 1;
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '^' => { blizzards.push((Point::new(x as i32, y as i32), Direction::Up)); },
                'v' => { blizzards.push((Point::new(x as i32, y as i32), Direction::Down)); },
                '<' => { blizzards.push((Point::new(x as i32, y as i32), Direction::Left)); },
                '>' => { blizzards.push((Point::new(x as i32, y as i32), Direction::Right)); },
                '#' => { walls.insert(Point::new(x as i32, y as i32)); }
                _ => ()
            }
        }
    }

    // Insert walls just beyond the start and end positions to keep from
    // trying to go around the outside.
    walls.insert(Point::new(1, -1));
    walls.insert(Point::new(width - 2, height));

    (blizzards, walls, width, height)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    position: Point,
    blizzards: Vec<(Point, Direction)>
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point {x,y}
    }

    fn dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
