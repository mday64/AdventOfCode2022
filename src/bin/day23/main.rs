use std::collections::{HashMap, HashSet};
use multiset::HashMultiSet;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day23/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let positions = parse_input(&input);

    let result1 = part1(positions.clone());
    println!("Part 1: {result1}");
    assert_eq!(result1, 3882);

    let result2 = part2(positions);
    println!("Part 2: {result2}");
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

// Do one round of movements.
// Return true if any elf moved.
fn one_round(positions: &mut HashSet<Point>, directions:&mut Vec<Direction>) -> bool {
    let mut moved = false;

    // Compute the proposed moves and positions
    let mut proposed_moves = HashMap::<Point, Point>::new();
    let mut proposed_positions = HashMultiSet::<Point>::new();

    for &Point{x,y} in positions.iter() {
        if !positions.contains(&Point{x: x-1, y: y-1}) &&
            !positions.contains(&Point{x,      y: y-1}) &&
            !positions.contains(&Point{x: x+1, y: y-1}) &&
            !positions.contains(&Point{x: x-1, y     }) &&
            !positions.contains(&Point{x: x+1, y     }) &&
            !positions.contains(&Point{x: x-1, y: y+1}) &&
            !positions.contains(&Point{x     , y: y+1}) &&
            !positions.contains(&Point{x: x+1, y: y+1})
        {
            // The elf stays where it is
            proposed_moves.insert(Point{x,y}, Point{x,y});
            proposed_positions.insert(Point{x,y});
            continue;
        }

        let mut new_x = x;
        let mut new_y = y;
        for dir in directions.iter().copied() {
            match dir {
                Direction::North => {
                    if !positions.contains(&Point{x: x-1, y: y-1}) &&
                        !positions.contains(&Point{x, y: y-1}) &&
                        !positions.contains(&Point{x: x+1, y: y-1})
                    {
                        new_y = y - 1;
                        break;
                    }
                },
                Direction::South => {
                    if !positions.contains(&Point{x: x-1, y: y+1}) &&
                        !positions.contains(&Point{x, y: y+1}) &&
                        !positions.contains(&Point{x: x+1, y: y+1})
                    {
                        new_y = y + 1;
                        break;
                    }
                },
                Direction::West => {
                    if !positions.contains(&Point{x: x-1, y: y-1}) &&
                        !positions.contains(&Point{x: x-1, y}) &&
                        !positions.contains(&Point{x: x-1, y: y+1})
                    {
                        new_x = x - 1;
                        break;
                    }
                },
                Direction::East => {
                    if !positions.contains(&Point{x: x+1, y: y-1}) &&
                        !positions.contains(&Point{x: x+1, y}) &&
                        !positions.contains(&Point{x: x+1, y: y+1})
                    {
                        new_x = x + 1;
                        break;
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
            if old_pos != new_pos {
                moved = true;
            }
            positions.insert(new_pos);
        } else {
            positions.insert(old_pos);
        }
    }

    // Rotate the order of directions to consider for next round
    let dir = directions.remove(0);
    directions.push(dir);

    moved
}

fn parse_input(input: &str) -> HashSet<Point> {
    let mut positions = HashSet::<Point>::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                positions.insert(Point{x: x as i32, y: y as i32});
            }
        }
    }
    positions
}

#[derive(Hash, Clone, PartialEq, Eq)]
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
