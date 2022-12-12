use std::collections::HashMap;
use pathfinding::directed::astar::astar;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day12/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    //
    // Part 1
    //
    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 504);
}

fn part1(input: &str) -> i32 {
    let input = parse_input(&input);
    astar(
        &input.starting_point,
        |node| input.neighbors(node),
        |node| input.heuristic(node),
        |node| node == &input.ending_point
    ).unwrap().1
}

// The input could be represented as a 2-D array of heights,
// but a HashMap makes it a little easier to deal with edges
// (where some potential neighbor coordinates are not valid).
type Coord = (i32, i32);
struct Input {
    starting_point: Coord,
    ending_point: Coord,
    heights: HashMap<Coord, u32>
}
fn parse_input(input: &str) -> Input {
    let mut heights = HashMap::new();
    let mut starting_point = None;
    let mut ending_point = None;

    for (row, line) in input.lines().enumerate() {
        let row = row as i32;
        for (col, ch) in line.chars().enumerate() {
            let col = col as i32;
            match ch {
                'S' => {
                    heights.insert((row, col), 0);
                    starting_point = Some((row, col));
                },
                'E' => {
                    heights.insert((row, col), 25);
                    ending_point = Some((row, col));
                },
                'a'..='z' => {
                    heights.insert((row, col), ch as u32 - 'a' as u32);
                }
                _ => panic!("Invalid input")
            }
        }
    }
    
    Input {
        starting_point: starting_point.unwrap(),
        ending_point: ending_point.unwrap(),
        heights
    }
}

impl Input {
    fn neighbors(&self, node: &Coord) -> Vec<(Coord, i32)> {
        let (row, col) = *node;
        let current_height = self.heights[node];
        let mut result = Vec::new();
        //TODO: Use filter_map?
        for other in [(row-1, col), (row+1, col), (row, col-1), (row, col+1)] {
            if let Some(other_height) = self.heights.get(&other) {
                if *other_height <= current_height + 1 {
                    result.push((other, 1));
                }
            }
        }
        result
    }
    fn heuristic(&self, node: &Coord) -> i32 {
        (node.0 - self.ending_point.0).abs() + (node.1 - self.ending_point.1).abs()
    }
}

#[test]
fn test_part1() {
    let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi\n";
    assert_eq!(part1(input), 31);
}