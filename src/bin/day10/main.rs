fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day10/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    //
    // Part 1
    //
    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 12460);
}

fn part1(input: &str) -> i32 {
    let mut result = 0;
    let x_values = run_program(&input);
    for i in [20, 60, 100, 140, 180, 220] {
        result += i as i32 * x_values[i-1];
    }
    result
}

fn run_program(program: &str) -> Vec<i32> {
    let mut result = Vec::new();
    let mut x = 1;
    for line in program.lines() {
        if line == "noop" {
            result.push(x);
        } else if line.starts_with("addx ") {
            let (_, v) = line.split_once(' ').unwrap();
            result.push(x);
            result.push(x);
            x += v.parse::<i32>().unwrap();
        }
    }
    result
}
