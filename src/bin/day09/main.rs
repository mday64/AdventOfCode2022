use std::collections::HashSet;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day09/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    //
    // Part 1
    //
    // Figuring out how to express the movement rules is the key here.
    // I think I need to keep a vertical and horizontal distance.
    // As long as each is at most 1, the head and tail are touching,
    // and the tail doesn't need to move.  If both are non-zero, then
    // the tail moves diagonally; otherwise, it moves in one dimension.
    //
    // The per-line movement instructions can move the head more than
    // one space.  I **think** I can move the head those multiple
    // positions all at once, and then have the tail catch up (as
    // opposed to simulating one square at a time).
    //
    // The final answer means we need to keep track of visited positions.
    //
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head_x = 0;
    let mut head_y = 0;
    let mut tail_x = 0;
    let mut tail_y = 0;
    visited.insert((0, 0));

    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse::<i32>().unwrap();
        match dir {
            "U" => head_y -= count,
            "D" => head_y += count,
            "L" => head_x -= count,
            "R" => head_x += count,
            _ => panic!("Invalid movement")
        }

        // Now move the tail to catch up
        while (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1 {
            tail_x += (head_x - tail_x).signum();
            tail_y += (head_y - tail_y).signum();
            visited.insert((tail_x, tail_y));
        }
    }
    let result1 = visited.len();
    println!("Part 1: {}", result1);
    assert_eq!(result1, 6486);
}
