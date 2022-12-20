fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day20/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let numbers = parse_numbers(&input);

    let result1 = part1(&numbers);
    println!("Part 1: {}", result1);
    assert!(result1 < 15077);
    assert!(result1 > 13972);
    // assert_eq!(result1, 0);
}

fn part1(numbers: &[i32]) -> i32 {
    let len = numbers.len() as i32;
    let mut mixed = Vec::from(numbers);
    for number in numbers {
        // Move number by number places
        // let shift_by = *number % len;
        let shift_by = *number;

        // Figure out what position `number` is currently at
        let src = mixed.iter().position(|v| v == number).unwrap() as i32;
        let dest = (src + shift_by).rem_euclid(len - 1);
        assert_eq!(mixed.remove(src as usize), *number);
        mixed.insert(dest as usize, *number);
        // println!("{:?}", mixed);
    }

    let zero_pos = mixed.iter().position(|&v| v == 0).unwrap();
    mixed[(zero_pos + 1000) % numbers.len()] +
    mixed[(zero_pos + 2000) % numbers.len()] +
    mixed[(zero_pos + 3000) % numbers.len()]
}

fn parse_numbers(s: &str) -> Vec<i32> {
    s.lines().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()
}

#[test]
fn test_part1() {
    let input = "1\n2\n-3\n3\n-2\n0\n4\n";
    let numbers = parse_numbers(input);
    // println!("{:?}", numbers);
    assert_eq!(part1(&numbers), 3);
}
