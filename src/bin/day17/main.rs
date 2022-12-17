fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day17/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let input = input.trim_end();

    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    // assert_eq!(result1, 0);
}

fn part1(input: &str) -> usize {
    todo!()
}

#[test]
fn test_part1() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(input), 3068);
}
