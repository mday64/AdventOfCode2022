use std::{fmt::Debug, ops::{Add, Sub}, str::FromStr};
use aoc2022::RangeSet;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day15/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input, 2_000_000);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 4985193);

    let result2 = part2(&input, 4_000_000);
    println!("Part 2: {}", result2);
    assert_eq!(result2, 11583882601918);
}

fn part1(input: &str, y: i32) -> i32 {
    let mut ranges = RangeSet::new();
    let pairs = input.lines().map(parse_line::<i32>).collect::<Vec<_>>();
    for (sensor, beacon) in pairs.iter() {
        let dist = sensor.distance_to(beacon);
        let dist_to_y = (sensor.1 - y).abs();
        // println!("sensor = {sensor:?}, beacon = {beacon:?}, dist = {dist}, dist_y = {dist_to_y}");
        if dist >= dist_to_y {
            // There is at least one point on line `y` that is within `dist`
            let min_x = sensor.0 - (dist - dist_to_y);
            let max_x = sensor.0 + (dist - dist_to_y);
            ranges.insert(min_x .. max_x+1);
        }
    }

    // Now remove any beacons on line `y`
    for (_, beacon) in pairs.iter() {
        if beacon.1 == y {
            ranges.remove(beacon.0 .. beacon.0+1);
        }
    }

    ranges.len()
}

fn part1_range_set(input: &str, y: i64) -> RangeSet<i64> {
    let mut ranges = RangeSet::new();
    let pairs = input.lines().map(parse_line::<i64>).collect::<Vec<_>>();
    for (sensor, beacon) in pairs.iter() {
        let dist = sensor.distance_to(beacon);
        let dist_to_y = (sensor.1 - y).abs();
        // println!("sensor = {sensor:?}, beacon = {beacon:?}, dist = {dist}, dist_y = {dist_to_y}");
        if dist >= dist_to_y {
            // There is at least one point on line `y` that is within `dist`
            let min_x = sensor.0 - (dist - dist_to_y);
            let max_x = sensor.0 + (dist - dist_to_y);
            ranges.insert(min_x .. max_x+1);
        }
    }

    // Now add any beacons on line `y`, which can't be the location
    // of the distress signal.
    for (_, beacon) in pairs.iter() {
        if beacon.1 == y {
            ranges.insert(beacon.0 .. beacon.0+1);
        }
    }

    ranges
}

fn part2(input: &str, upper_y: i64) -> i64 {
    // How do I solve this?  I can't try all 4,000,000 * 4,000,000
    // possible coordinates.

    // Let's try brute force.  Let's apply part1 to all possible Y
    // values and see which one ends up as 2 ranges.
    for y in 0 ..= upper_y {
        // print!("{y}\r");
        let mut ranges = part1_range_set(input, y);
        ranges.intersect(0..upper_y+1);
        if ranges.ranges.len() > 1 {
            assert_eq!(ranges.ranges.len(), 2);
            let x = ranges.ranges[0].end;
            // println!("ranges: {ranges:?} => x={x}, y={y}");
            return 4_000_000 * x + y;
        }
    }
    panic!("No solution found!");
}

#[derive(Debug, PartialEq, Eq)]
struct Point<T>(T, T);

impl<T> Point<T>
where T: Copy + Sub<Output=T> + Add<Output=T> + Ord
{
    fn distance_to(&self, other: &Self) -> T {
        let min0 = self.0.min(other.0);
        let max0 = self.0.max(other.0);
        let min1 = self.1.min(other.1);
        let max1 = self.1.max(other.1);
        (max0 - min0) + (max1 - min1)
    }
}

//
// Parse a line that looks like:
//  Sensor at x=2, y=18: closest beacon is at x=-2, y=15
// and return the Point for the sensor and beacon (in that order)
//
fn parse_line<T>(mut line: &str) -> (Point<T>, Point<T>)
where T: FromStr, <T as FromStr>::Err: Debug
{
    let mut left = line.find("x=").unwrap();
    let mut right = line.find(",").unwrap();
    let mut x = line[left+2..right].parse::<T>().unwrap();
    line = &line[right..];

    left = line.find("y=").unwrap();
    right = line.find(":").unwrap();
    let mut y = line[left+2..right].parse::<T>().unwrap();
    line = &line[right..];

    let sensor = Point(x,y);

    left = line.find("x=").unwrap();
    right = line.find(",").unwrap();
    x = line[left+2..right].parse::<T>().unwrap();
    line = &line[right..];

    left = line.find("y=").unwrap();
    y = line[left+2..].parse::<T>().unwrap();

    let beacon = Point(x,y);

    (sensor, beacon)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
        Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
        Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
        Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
        Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
        Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
        Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
        Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
        Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
        Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
        Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
        Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
        Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
        Sensor at x=20, y=1: closest beacon is at x=15, y=3\n";
    
    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Sensor at x=2, y=18: closest beacon is at x=-2, y=15"),
            (Point(2,18), Point(-2,15))
        );
        assert_eq!(
            parse_line("Sensor at x=20, y=14: closest beacon is at x=25, y=17"),
            (Point(20,14), Point(25,17))
        );
    }
    
    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE, 10), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE, 20), 56000011);
    }
}
