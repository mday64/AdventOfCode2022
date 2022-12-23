use std::collections::{HashMap, HashSet};
use multiset::HashMultiSet;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day23/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {result1}");
    assert!(result1 < 4374);
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
fn part1(input: &str) -> usize {
    // Parse the input to produce the initial state
    let mut positions = HashSet::<Point>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                positions.insert(Point{x: x as i32, y: y as i32});
            }
        }
    }

    use Direction::*;
    let mut directions = vec![North, South, West, East];
    for _ in 0 .. 10 {
        // Compute the proposed moves and positions
        let mut proposed_moves = HashMap::<Point, Point>::new();
        let mut proposed_positions = HashMultiSet::<Point>::new();

        for &Point{x,y} in positions.iter() {
            let mut new_x = x;
            let mut new_y = y;
            for dir in directions.iter().copied() {
                match dir {
                    North => {
                        if !positions.contains(&Point{x: x-1, y: y-1}) &&
                           !positions.contains(&Point{x, y: y-1}) &&
                           !positions.contains(&Point{x: x+1, y: y-1})
                        {
                            new_y = y - 1;
                        }
                    },
                    South => {
                        if !positions.contains(&Point{x: x-1, y: y+1}) &&
                           !positions.contains(&Point{x, y: y+1}) &&
                           !positions.contains(&Point{x: x+1, y: y+1})
                        {
                            new_y = y + 1;
                        }
                    },
                    West => {
                        if !positions.contains(&Point{x: x-1, y: y-1}) &&
                           !positions.contains(&Point{x: x-1, y}) &&
                           !positions.contains(&Point{x: x-1, y: y+1})
                        {
                            new_x = x - 1;
                        }
                    },
                    East => {
                        if !positions.contains(&Point{x: x+1, y: y-1}) &&
                           !positions.contains(&Point{x: x+1, y}) &&
                           !positions.contains(&Point{x: x+1, y: y+1})
                        {
                            new_x = x + 1;
                        }
                    },
                }
            }
            proposed_moves.insert(Point{x,y}, Point{x: new_x, y: new_y});
            proposed_positions.insert(Point{x: new_x, y: new_y});
        }

        // Update positions
        positions.clear();
        for (old_pos, new_pos) in proposed_moves.into_iter() {
            if proposed_positions.count_of(&new_pos) == 1 {
                positions.insert(new_pos);
            } else {
                positions.insert(old_pos);
            }
        }

        // Rotate the order of directions to consider for next round
        let dir = directions.remove(0);
        directions.push(dir);
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

#[derive(Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East
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
    assert_eq!(part1(input), 110);
}