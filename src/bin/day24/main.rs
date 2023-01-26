use std::ops::Add;
use pathfinding::prelude::astar;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day24/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let (width, height, rows, cols) = parse_input(&input);

    let start = Point::new(0, -1);
    let end = Point::new(width-1, height);
    
    let success_start = |state: &State| state.position == start;
    let heuristic_start = |state: &State| state.position.dist(&start);
    let success_end = |state: &State| state.position == end;
    let heuristic_end = |state: &State|  state.position.dist(&end);

    let successors = |state: &State| -> Vec<(State, i32)> {
        let time = state.time + 1;
        let mut result = Vec::new();

        let mut ok_up = true;
        let mut ok_down = true;
        let mut ok_left = true;
        let mut ok_right = true;
        let mut ok_center = true;

        let x = state.position.x;
        let y = state.position.y;

        if y == -1 {
            // assert_eq!(x, 0);
            ok_up = false;
            ok_right = false;
            ok_left = false;
        }

        if y == height {
            // assert_eq!(x, width-1);
            ok_down = false;
            ok_left = false;
            ok_right = false;
        }

        for blizzard in &cols[x as usize] {
            let yy = blizzard.position(time, height);
            if yy == y - 1 {
                ok_up = false;
            }
            if yy == y {
                ok_center = false;
            }
            if yy == y + 1 {
                ok_down = false;
            }
        }

        if y >= 0 && y < height {
            for blizzard in &rows[y as usize] {
                let xx = blizzard.position(time, width);
                if xx == x - 1 {
                    ok_left = false;
                }
                if xx == x {
                    ok_center = false;
                }
                if xx == x + 1 {
                    ok_right = false;
                }
            }
        }

        if ok_up && y > 0 {
            if rows[(y-1) as usize].iter().any(|b| b.position(time, width) == x) {
                ok_up = false;
            }
        } else {
            ok_up = false;
        }

        if ok_down && y < height - 1 {
            if rows[(y+1) as usize].iter().any(|b| b.position(time, width) == x) {
                ok_down = false;
            }
        } else {
            ok_down = false;
        }

        if ok_left && x > 0 {
            if cols[(x-1) as usize].iter().any(|b| b.position(time, height) == y) {
                ok_left = false;
            }
        } else {
            ok_left = false;
        }

        if ok_right && x < width - 1 {
            if cols[(x+1) as usize].iter().any(|b| b.position(time, height) == y) {
                ok_right = false;
            }
        } else {
            ok_right = false;
        }

        if ok_up || (x == 0 && y == 0) {
            let position = state.position + (0, -1);
            // assert!(position == start || (_in_bounds(&position) && _empty_at(&position, time)));
            result.push((State{position, time}, 1));
        }
        if ok_down || (x == width-1 && y == height-1) {
            let position = state.position + (0,1);
            // assert!(position == end || (_in_bounds(&position) && _empty_at(&position, time)));
            result.push((State{position, time}, 1));
        }
        if ok_left {
            let position = state.position + (-1, 0);
            // assert!(_in_bounds(&position));
            // assert!(_empty_at(&position, time));
            result.push((State{position, time}, 1));
        }
        if ok_right {
            let position = state.position + (1, 0);
            // assert!(_in_bounds(&position));
            // assert!(_empty_at(&position, time));
            result.push((State{position, time}, 1));
        }
        if ok_center {
            result.push((State{position: state.position, time}, 1));
        }

        result
    };

    //
    // Part 1
    //
    let now = std::time::Instant::now();
    let (_, steps1) = astar(
            &State{ position: start, time: 0},
            successors, heuristic_end, success_end
        ).unwrap();
    let duration = now.elapsed();
    println!("Part 1: {steps1} in {duration:?}");
    assert_eq!(steps1, 299);

    //
    // Part 2
    //
    let now = std::time::Instant::now();
    let (_, steps2) = astar(
            &State{ position: end, time: steps1 },
            successors, heuristic_start, success_start
        ).unwrap();
    let (_, steps3) = astar(
            &State{ position: start, time: steps1 + steps2 },
            successors, heuristic_end, success_end
        ).unwrap();
    let result2 = steps1 + steps2 + steps3;
    let duration = now.elapsed();
    println!("Part 2: {result2} in {duration:?}");
    assert_eq!(result2, 899);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    position: Point,
    time: i32
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
fn parse_input(input: &str) -> (i32, i32, Vec<Vec<Blizzard>>, Vec<Vec<Blizzard>>) {
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
        let mut answer = (self.initial + time * self.direction) % modulus;
        if answer < 0 {
            answer += modulus;
        }
        answer
    }
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

impl Add<(i32, i32)> for Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl Add<(i32, i32)> for &Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point::new(self.x + rhs.0, self.y + rhs.1)
    }
}
