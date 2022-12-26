use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day08/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    
    let (num_rows, num_cols, grid) = parse_input(&input);

    let result1 = part1(num_rows, num_cols, &grid);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 1854);

    let result2 = part2(num_rows, num_cols, &grid);
    println!("Part 2: {}", result2);
    assert_eq!(result2, 527340);
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
fn part1(num_rows: usize, num_cols: usize, grid: &[Vec<i8>]) -> usize {
    let mut visible = HashSet::<(usize, usize)>::new();
    #[allow(clippy::needless_range_loop)]
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
        #[allow(clippy::needless_range_loop)]
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

//
// Part 2
//
// I could brute force look at every position on the grid, and
// compute its scenic score, then just take the max.  Since the full
// input is 99x99, that might be reasonable.
//
// I can only guess that there is a more efficient solution.  Perhaps
// starting with the tallest trees because they are more likely to
// have a higher scenic score?  But might a shorter tree be in a
// much better position (with lots of lower trees nearby), that ends
// up with a better score?
//
fn part2(num_rows: usize, num_cols: usize, grid: &Vec<Vec<i8>>) -> u32 {
    (0..num_rows).cartesian_product(0..num_cols)
        .map(|(row, col)| scenic_score(grid, row, col))
        .max().unwrap()
}

fn scenic_score(grid: &Vec<Vec<i8>>, row: usize, col: usize) -> u32 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let height = grid[row][col];
    let mut score = 1;
    let mut dir_score;

    // Look right
    dir_score = 0;
    for c in (col+1)..num_cols {
        dir_score += 1;
        if grid[row][c] >= height {
            break;
        }
    }
    score *= dir_score;
    
    // Look left
    dir_score = 0;
    for c in (0..col).rev() {
        dir_score += 1;
        if grid[row][c] >= height {
            break;
        }
    }
    score *= dir_score;

    // Look down
    dir_score = 0;
    #[allow(clippy::needless_range_loop)]
    for r in (row+1)..num_rows {
        dir_score += 1;
        if grid[r][col] >= height {
            break;
        }
    }
    score *= dir_score;

    // Look up
    dir_score = 0;
    for r in (0..row).rev() {
        dir_score += 1;
        if grid[r][col] >= height {
            break;
        }
    }
    score *= dir_score;

    score
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

#[test]
fn test_part2() {
    let grid = vec![
        vec![3,0,3,7,3],
        vec![2,5,5,1,2],
        vec![6,5,3,3,2],
        vec![3,3,5,4,9],
        vec![3,5,3,9,0],
    ];

    assert_eq!(scenic_score(&grid, 1, 2), 4);
    assert_eq!(scenic_score(&grid, 3, 2), 8);
    assert_eq!(part2(5, 5, &grid), 8);
}
