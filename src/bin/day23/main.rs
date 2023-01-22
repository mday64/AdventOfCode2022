use fxhash::FxHashSet as HashSet;
use std::ops::{Add, AddAssign, Sub, Mul};

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day23/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let positions = parse_input(&input);

    let now = std::time::Instant::now();
    let result1 = part1(positions.clone());
    let duration = now.elapsed().as_secs_f64();
    println!("Part 1: {result1} (in {duration} seconds)");
    assert_eq!(result1, 3882);

    let now = std::time::Instant::now();
    let result2 = part2(positions);
    let duration = now.elapsed().as_secs_f64();
    println!("Part 2: {result2} (in {duration} seconds)");
    assert_eq!(result2, 1116);
}

//
// Part 1
//
// How should we represent the state?
//      A HashSet or Vec of elf positions?
//          We need to be able to iterate over them, and also see if
//          a given position is occupied.
//      A multiset (bag) for proposed new positions
//          So we know if multiple elves proposed that position.
//      A cache from current positon to proposed position?
//          To potentially speed up selecting the new position, and we
//          don't need to go through the logic for proposing a position twice.
//
//  How should we keep track of the order to consider new positions?
//      Perhaps just a vec![0,1,2,3] that we iterate over and match
//      on the value to do the direction-dependent logic.
//
fn part1(mut positions: HashSet<Point>) -> usize {
    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East
    ];

    for _ in 0 .. 10 {
        one_round(&mut positions, &mut directions);
    }

    // Compute how many empty spaces within the tightest bounding rectangle
    let mut positions_iter = positions.iter();
    let Point{x: mut min_x, y: mut min_y} = positions_iter.next().unwrap();
    let mut max_x = min_x;
    let mut max_y = min_y;
    for p in positions_iter {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }
    (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize - positions.len()
}

fn part2(mut positions: HashSet<Point>) -> u32 {
    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East
    ];

    let mut rounds = 0;
    loop {
        rounds += 1;
        let moved = one_round(&mut positions, &mut directions);
        if !moved {
            break;
        }
    }
    
    rounds
}

// Neighbor offsets, from upper left, going clockwise, plus the first one repeated
// (which will make it easier to get the three neighbors on a side)
const NEIGHBORS:[(i32, i32);9] = [(-1,-1), (0,-1), (1,-1), (1,0), (1,1), (0,1), (-1,1), (-1, 0), (-1,-1)];

// Do one round of movements.
// Return true if any elf moved.
//
// PERFORMANCE
//
// One way to potentially speed it up is to grab all 8 neighbors, and store
// them temporarily.  First check to see if any are occupied.  If so, then
// recheck 3 at a time to find the proposed direction.
//
// ----
//
// According to <https://www.reddit.com/r/adventofcode/comments/zt6xz5/comment/j1dq8oj/>,
// at most two elves can propose to move to the same location, and they must
// come from opposite directions.  (If they came from right angles, then the
// elves would be adjacent diagonally, and therefore wouldn't propose that
// destination.)
//
// That means we don't need to calculate all of the proposed moves before
// actually moving the elves.  As long as we have separate sets for previous
// and next positions, and determine and elf's next position based on the
// previous set, if we find an elf already exists in our proposed destination,
// we can just push the other elf back to its previous location and not move
// ourselves.
//
fn one_round(positions: &mut HashSet<Point>, directions:&mut Vec<Direction>) -> bool {
    let mut moves = 0;

    let previous = positions.clone();
    positions.clear();

    // Compute the proposed moves and positions
    for elf in previous.iter() {
        let mut new_elf = *elf;

        // Determine its proposed position (using `previous`)
        if NEIGHBORS[..8].iter().all(|n| !previous.contains(&(elf+n)))
        {
            // The elf stays where it is
        } else {
            for dir in directions.iter().copied() {
                let neighbors_index = (dir as usize) * 2;
                if NEIGHBORS[neighbors_index..][..3].iter().all(|n| !previous.contains(&(elf+n))) {
                    new_elf += NEIGHBORS[neighbors_index + 1];
                    break;
                }
            }
        }

        if positions.insert(new_elf) {
            if new_elf != *elf {
                moves += 1;
            }
        } else {
            // We collided with an elf coming from the opposite direction.
            // Move it back (in the same direction)
            positions.remove(&new_elf);
            positions.insert(new_elf * 2 - *elf);
            moves -= 1;
            // Leave us in our original position
            positions.insert(*elf);
        }
    }

    // Rotate the order of directions to consider for next round
    let dir = directions.remove(0);
    directions.push(dir);

    moves > 0
}

fn parse_input(input: &str) -> HashSet<Point> {
    let mut positions = HashSet::<Point>::default();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                positions.insert(Point{x: x as i32, y: y as i32});
            }
        }
    }
    positions
}

#[allow(dead_code)]
fn print_elves(positions: &HashSet<Point>) {
    let min_x = positions.iter().map(|p| p.x).min().unwrap();
    let max_x = positions.iter().map(|p| p.x).max().unwrap();
    let min_y = positions.iter().map(|p| p.y).min().unwrap();
    let max_y = positions.iter().map(|p| p.y).max().unwrap();

    for y in min_y ..= max_y {
        for x in min_x ..= max_x {
            if positions.contains(&Point{x,y}) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<&(i32,i32)> for &Point {
    type Output = Point;

    fn add(self, rhs: &(i32,i32)) -> Self::Output {
        Point{ x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

impl Add<(i32,i32)> for Point {
    type Output = Point;

    fn add(self, rhs: (i32,i32)) -> Self::Output {
        Point{x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

impl AddAssign<(i32,i32)> for Point {
    fn add_assign(&mut self, rhs: (i32,i32)) {
        self.x += rhs.0;
        self.y += rhs.1;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point{ x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Point{ x: self.x * rhs, y: self.y * rhs }
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[test]
fn test_part1() {
    let input = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";
    let positions = parse_input(input);
    assert_eq!(part1(positions), 110);
}

#[test]
fn test_part2() {
    let input = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";
    let positions = parse_input(input);
    assert_eq!(part2(positions), 20);
}

#[test]
fn test_rounds_mini() {
    let input = "\
    .....\n\
    ..##.\n\
    ..#..\n\
    .....\n\
    ..##.\n\
    .....\n";

    let mut positions = parse_input(input);
    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East
    ];
    println!("== Initial Position ==");
    print_elves(&positions);
    for i in 1..=3 {
        println!("== Round {i} ==");
        one_round(&mut positions, &mut directions);
        print_elves(&positions);
    }
}
