use std::collections::HashMap;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day14/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 858);
}

fn part1(input: &str) -> usize {
    // Parse the input
    let mut cells = HashMap::<Coord, Cell>::new();
    for line in input.lines() {
        fill_path(&mut cells, line);
    }

    // Find the greatest Y value in `cells`
    let max_y = cells.keys().map(|(_,y)| y).max().copied().unwrap();
    let min_x = cells.keys().map(|(x,_)| x).min().copied().unwrap();
    let max_x = cells.keys().map(|(x,_)| x).max().copied().unwrap();

    // Start dropping units of sand
    let mut done = false;
    while !done {
        // print!("\x1B[2J");
        // show_cells(&cells, 400, 520, 0, 90);
        // std::thread::sleep(std::time::Duration::from_secs(1));
        // Starting position of sand
        let mut x = 500;
        let mut y = 0;

        // Move until it settles or falls off the bottom
        while y <= max_y {
            if cells.get(&(x, y+1)).is_none() {
                // Must be air, keep falling down
                y += 1;
            } else if cells.get(&(x-1, y+1)).is_none() {
                // Go down and left
                x -= 1;
                y += 1;
            } else if cells.get(&(x+1, y+1)).is_none() {
                // Go down and right
                x += 1;
                y += 1;
            } else {
                // Sand settles here
                // println!("({x}, {y})");
                cells.insert((x,y), Cell::Sand);
                break;
            }
        }
        if y > max_y {
            // There was no place to settle, so we must be done
            done = true;
        }
    }

    // print!("\x1B[2J");
    // show_cells(&cells, 400, 520, 0, max_y);
    
    cells.values().filter(|v| v == &&Cell::Sand).count()
}

type Coord = (i32, i32);

#[derive(PartialEq, Eq)]
enum Cell {
    Rock,
    Sand
}

fn fill_path(cells: &mut HashMap<Coord, Cell>, line: &str) {
    let mut coords = line.split(" -> ");
    let (mut x, mut y) = parse_coord(coords.next().unwrap());
    cells.insert((x,y), Cell::Rock);
    for point in coords {
        let (nx, ny) = parse_coord(point);
        let dx = (nx - x).signum();
        let dy = (ny - y).signum();
        while x != nx || y != ny {
            x += dx;
            y += dy;
            cells.insert((x, y), Cell::Rock);
        }
    }
}

fn parse_coord(coord: &str) -> Coord {
    let (x, y) = coord.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

fn show_cells(cells: &HashMap<Coord, Cell>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
    for y in min_y ..= max_y {
        for x in min_x ..= max_x {
            match cells.get(&(x, y)) {
                None => print!(" "),
                Some(Cell::Rock) => print!("#"),
                Some(Cell::Sand) => print!("o"),
            }
        }
        println!();
    }
}

#[test]
fn test_part1() {
    let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";
    assert_eq!(part1(input), 24);
}
