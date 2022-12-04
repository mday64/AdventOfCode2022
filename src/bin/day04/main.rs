use std::ops::RangeInclusive;
use regex::Regex;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day04/input.txt".into());
    let input = std::fs::read_to_string(path)
        .expect("Can't read input");

    //
    // Parsing the input
    //
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let range_pairs = input.lines().map(|line| {
        let captures = re.captures(line).expect("Can't parse line");
        let s1 = captures[1].parse::<u32>().unwrap();
        let e1 = captures[2].parse::<u32>().unwrap();
        let s2 = captures[3].parse::<u32>().unwrap();
        let e2 = captures[4].parse::<u32>().unwrap();
        (RangeInclusive::new(s1, e1), RangeInclusive::new(s2, e2))
    }).collect::<Vec<_>>();

    //
    // Part 1
    //
    // One range fully contains another range if it contains both the start and end
    // of the other range.
    //
    let result1 = range_pairs.iter().filter(|(r1, r2)|
        (r1.contains(r2.start()) && r1.contains(r2.end())) ||
        (r2.contains(r1.start()) && r2.contains(r1.end()))
    ).count();
    println!("Part 1: {}", result1);
    assert_eq!(result1, 441);

    //
    // Part 2
    //
    // One range overlaps another if it contains the start *or* end of the other.
    //
    let result2 = range_pairs.iter().filter(|(r1, r2)|
        (r1.contains(r2.start()) || r1.contains(r2.end())) ||
        (r2.contains(r1.start()) || r2.contains(r1.end()))
    ).count();
    println!("Part 2: {}", result2);
    assert_eq!(result2, 861);
}
