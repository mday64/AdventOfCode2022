use std::collections::HashSet;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day15/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input, 2_000_000);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 4985193);
}

fn part1(input: &str, y: i32) -> usize {
    let mut positions = HashSet::<i32>::new();
    let pairs = input.lines().map(parse_line).collect::<Vec<_>>();
    for (sensor, beacon) in pairs.iter() {
        let dist = sensor.distance_to(beacon);
        let dist_to_y = (sensor.1 - y).abs();
        // println!("sensor = {sensor:?}, beacon = {beacon:?}, dist = {dist}, dist_y = {dist_to_y}");
        if dist >= dist_to_y {
            // There is at least one point on line `y` that is within `dist`
            let min_x = sensor.0 - (dist - dist_to_y);
            let max_x = sensor.0 + (dist - dist_to_y);
            for x in min_x ..= max_x {
                // println!("    x={x}");
                positions.insert(x);
            }
        }
    }

    // Now remove any beacons on line `y`
    for (_, beacon) in pairs.iter() {
        if beacon.1 == y {
            positions.remove(&beacon.0);
        }
    }
    positions.len()
}

#[derive(Debug, PartialEq, Eq)]
struct Point(i32, i32);

impl Point {
    fn distance_to(&self, other: &Self) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

//
// Parse a line that looks like:
//  Sensor at x=2, y=18: closest beacon is at x=-2, y=15
// and return the Point for the sensor and beacon (in that order)
//
fn parse_line(mut line: &str) -> (Point, Point) {
    let mut left = line.find("x=").unwrap();
    let mut right = line.find(",").unwrap();
    let mut x = line[left+2..right].parse::<i32>().unwrap();
    line = &line[right..];

    left = line.find("y=").unwrap();
    right = line.find(":").unwrap();
    let mut y = line[left+2..right].parse::<i32>().unwrap();
    line = &line[right..];

    let sensor = Point(x,y);

    left = line.find("x=").unwrap();
    right = line.find(",").unwrap();
    x = line[left+2..right].parse::<i32>().unwrap();
    line = &line[right..];

    left = line.find("y=").unwrap();
    y = line[left+2..].parse::<i32>().unwrap();

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
}
