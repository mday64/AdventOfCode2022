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
    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 6486);

    let result2 = part2(&input);
    println!("Part 2: {}", result2);
    assert_eq!(result2, 2678);
}

fn part1(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut rope = vec![(0,0), (0, 0)];

    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse::<i32>().unwrap();
        for _ in 0..count {
            step_head(&mut rope, dir);
            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn part2(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));
    let mut rope = vec![(0,0); 10];

    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse::<i32>().unwrap();
        for _ in 0..count {
            step_head(&mut rope, dir);
            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

type Knot = (i32, i32);
type Rope = Vec<Knot>;

fn step_head(rope: &mut Rope, dir: &str) {
    // Move the head
    let (mut head_x, mut head_y) = rope[0];
    match dir {
        "U" => head_y -= 1,
        "D" => head_y += 1,
        "L" => head_x -= 1,
        "R" => head_x += 1,
        _ => panic!("Invalid movement")
    }
    rope[0] = (head_x, head_y);

    // Cause the rest of the knots to catch up as needed
    for i in 0..(rope.len() - 1) {
        let (head_x, head_y) = rope[i];
        let (mut tail_x, mut tail_y) = rope[i+1];
        // Move the next knot to catch up
        if (head_x - tail_x).abs() > 1 || (head_y - tail_y).abs() > 1 {
            tail_x += (head_x - tail_x).signum();
            tail_y += (head_y - tail_y).signum();
        }
        rope[i+1] = (tail_x, tail_y);
    }
}
