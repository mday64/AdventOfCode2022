use std::ops::RangeInclusive;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day04/input.txt".into());
    let input = std::fs::read_to_string(path)
        .expect("Can't read input");

    //
    // Parsing the input
    //
    let mut range_pairs: Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> = Vec::new();
    for line in input.lines() {
        let mut ranges = line.split(',');
        let range1_str = ranges.next().expect("Can't get first range");
        let range2_str = ranges.next().expect("Can't get second range");

        let mut ends = range1_str.split('-');
        let start = ends.next().expect("Can't get start of first range");
        let end = ends.next().expect("Can't get end of first range");
        let range1 = RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap());

        let mut ends = range2_str.split('-');
        let start = ends.next().expect("Can't get start of first range");
        let end = ends.next().expect("Can't get end of first range");
        let range2 = RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap());

        range_pairs.push((range1, range2));
    }

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
