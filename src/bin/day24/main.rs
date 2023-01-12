use std::collections::HashSet;
use std::ops::{Add, AddAssign};
use pathfinding::prelude::astar;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day24/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let now = std::time::Instant::now();
    let result1 = part1(&input);
    let duration = now.elapsed().as_secs_f64();
    println!("Part 1: {result1} (in {duration} seconds)");
    assert_eq!(result1, 299);

    // let now = std::time::Instant::now();
    // let result2 = part2(&input);
    // let duration = now.elapsed().as_secs_f64();
    // println!("Part 2: {result2} (in {duration} seconds)");
    // assert_eq!(result2, 899);
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State2 {
    position: Point,
    time: i32
}

fn part1(input: &str) -> i32 {
    let (width, height, rows, cols) = parse_input2(input);

    let start = State2 { position: Point::new(0, -1), time: 0 };
    let end = Point::new(width-1, height);
    
    let in_bounds = |&Point{x,y}: &Point| -> bool {
        x >= 0 && x < width && y >= 0 && y < height
    };
    let empty_at = |&Point{x,y}: &Point, time: i32| -> bool {
        rows[y as usize].iter().all(|b| b.position(time, width) != x) &&
        cols[x as usize].iter().all(|b| b.position(time, height) != y)
    };

    let success = |state: &State2| state.position == end;
    let heuristic = |state: &State2| -> i32 {
        // Minimum cost: Manhattan distance to the end point
        state.position.dist(&end)
    };
    let successors = |state: &State2| -> Vec<(State2, i32)> {
        let x = state.position.x;
        let y = state.position.y;
        let time = state.time + 1;
        let mut result = Vec::new();

        for (dx,dy) in [(0,0), (-1,0), (1,0), (0, -1), (0, 1)] {
            let position = Point::new(x+dx, y+dy);
            if position == end || (in_bounds(&position) && empty_at(&position, time)) {
                result.push((State2{ position, time }, 1));
            }
        }

        result
    };
    let (_states, steps) = astar(&start, successors, heuristic, success).unwrap();
    steps
}

fn part2(input: &str) -> i32 {
    let (blizzards, walls, width, height) = parse_input(input);
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

//
//  parse_input
//
// Parse the input string and return information about the blizzard
// positions and directions, and the overall size of the area.
//
// For ease of computing future blizzard positions, the coordinate system
// is offset such that the first line of input is row -1, and the first
// column of each line is column -1.  That way, the inner area has rows
// numbered 0..height, and columns numbered 0..width.
//
// The output is a tuple:
//      0: width of the inner area (number of columns)
//      1: height of the inner area (number of rows)
//      2: rows of blizzards that move horizontally
//      3: columns of blizzards that move vertically
//
fn parse_input2(input: &str) -> (i32, i32, Vec<Vec<Blizzard>>, Vec<Vec<Blizzard>>) {
    // First, figure out the dimensions of the inner area
    let height = input.lines().count() - 2;
    let width = input.lines().next().unwrap().len() - 2;

    // Create the vectors to store the blizzards
    let mut rows = Vec::with_capacity(height);
    let mut cols = Vec::with_capacity(width);
    rows.resize_with(height, Vec::new);
    cols.resize_with(width, Vec::new);

    for (y,line) in input.lines().skip(1).take(height).enumerate() {
        for (x,ch) in line.chars().skip(1).take(width).enumerate() {
            match ch {
                '^' => cols[x].push(Blizzard::new(y as i32, -1)),
                'v' => cols[x].push(Blizzard::new(y as i32,  1)),
                '<' => rows[y].push(Blizzard::new(x as i32, -1)),
                '>' => rows[y].push(Blizzard::new(x as i32,  1)),
                '.' => (),
                _ => panic!("Unexpected character: {ch}")
            }
        }
    }

    (width as i32, height as i32, rows, cols)
}

struct Blizzard {
    initial: i32,
    direction: i32,      // -1 or +1
}

impl Blizzard {
    fn new(initial: i32, direction: i32) -> Self {
        Self { initial, direction }
    }

    fn position(&self, time: i32, modulus: i32) -> i32 {
        (self.initial + time * self.direction).rem_euclid(modulus)
    }
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
