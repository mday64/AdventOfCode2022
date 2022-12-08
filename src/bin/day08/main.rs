use std::collections::HashSet;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day08/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    
    let (num_rows, num_cols, grid) = parse_input(&input);

    let result1 = part1(num_rows, num_cols, &grid);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 1854);
}

//
// Part 1
//
// I'm going to pretend like I'm on the outside of each row/column,
// looking in.  Every time I encounter a taller tree, I note that
// it is visible.
//
// It's possible that a given tree is visible from multiple
// directions.  This means I can't just count how many trees
// are visible along a given direction.  I think I'm going to
// construct a HashSet of visible coordinates in order to remove
// duplicates.
//
fn part1(num_rows: usize, num_cols: usize, grid: &Vec<Vec<i8>>) -> usize {
    let mut visible = HashSet::<(usize, usize)>::new();
    for row in 0..num_rows {
        // Looking from left
        let mut tallest = -1;
        for col in 0..num_cols {
            if grid[row][col] > tallest {
                visible.insert((row, col));
                tallest = grid[row][col];
            }
        }

        // Looking from right
        tallest = -1;
        for col in (0..num_cols).rev() {
            if grid[row][col] > tallest {
                visible.insert((row, col));
                tallest = grid[row][col];
            }
        }
    }
    for col in 0..num_cols {
        // Looking down
        let mut tallest = -1;
        for row in 0..num_rows {
            if grid[row][col] > tallest {
                visible.insert((row, col));
                tallest = grid[row][col];
            }
        }

        // Looking up
        tallest = -1;
        for row in (0..num_rows).rev() {
            if grid[row][col] > tallest {
                visible.insert((row, col));
                tallest = grid[row][col];
            }
        }
    }

    visible.len()
}

fn parse_input(input: &str) -> (usize, usize, Vec<Vec<i8>>) {
    // Convert the input to a 2-dimensional grid.  The element type is
    // going to be i8 so that I can use -1 to represent off-grid values,
    // and not have to cast back and forth to unsigned.
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().len();
    let mut grid = Vec::<Vec<i8>>::with_capacity(num_rows);
    for line in input.lines() {
        let row = line.chars()
            .map(|c| c.to_digit(10).unwrap() as i8)
            .collect();
        grid.push(row);
    }
    (num_rows, num_cols, grid)
}
