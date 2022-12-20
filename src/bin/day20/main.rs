use std::collections::VecDeque;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day20/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let numbers = parse_numbers(&input);

    let result1 = part1(&numbers);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 1591);

    let result2 = part2(&numbers);
    println!("Part 2: {}", result2);
    // assert_eq!(result2, 1591);
}

fn part1(numbers: &[i32]) -> i32 {
    let mut mixed:VecDeque<(usize,i32)> = numbers.iter().copied().enumerate().collect();
    for index in 0..mixed.len() {
        let pos = mixed.iter().position(|&(i,_)| i == index).unwrap();
        let (i,number) = mixed.remove(pos).unwrap();
        assert_eq!(i, index);
        if number < 0 {
            // Moving the number to the left is equivalent to moving
            // the list to the right
            mixed.rotate_right((number.abs() as usize) % mixed.len());
        } else {
            // Moving the number to the right is equivalent to moving
            // the list to the left
            mixed.rotate_left((number as usize) % mixed.len());
        }
        mixed.insert(pos, (index,number));
    }

    let zero_pos = mixed.iter().position(|&(_,v)| v == 0).unwrap();
    mixed[(zero_pos + 1000) % numbers.len()].1 +
    mixed[(zero_pos + 2000) % numbers.len()].1 +
    mixed[(zero_pos + 3000) % numbers.len()].1
}

fn part2(numbers: &[i32]) -> i64 {
    let mut mixed:VecDeque<(usize,i64)> = numbers.iter().copied()
        .map(|v| v as i64 * 811589153)
        .enumerate().collect();
    for _ in 0..10 {
        for index in 0..mixed.len() {
            let pos = mixed.iter().position(|&(i,_)| i == index).unwrap();
            let (i,number) = mixed.remove(pos).unwrap();
            assert_eq!(i, index);
            if number < 0 {
                // Moving the number to the left is equivalent to moving
                // the list to the right
                mixed.rotate_right((number.abs() as usize) % mixed.len());
            } else {
                // Moving the number to the right is equivalent to moving
                // the list to the left
                mixed.rotate_left((number as usize) % mixed.len());
            }
            mixed.insert(pos, (index,number));
        }    
    }

    let zero_pos = mixed.iter().position(|&(_,v)| v == 0).unwrap();
    mixed[(zero_pos + 1000) % numbers.len()].1 +
    mixed[(zero_pos + 2000) % numbers.len()].1 +
    mixed[(zero_pos + 3000) % numbers.len()].1
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

#[test]
fn test_part2() {
    let input = "1\n2\n-3\n3\n-2\n0\n4\n";
    let numbers = parse_numbers(input);
    // println!("{:?}", numbers);
    assert_eq!(part2(&numbers), 1623178306);
}
