use std::{fmt::Debug, ops::{Add, Sub}, str::FromStr, collections::HashSet};
use aoc2022::RangeSet;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day15/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let input = input.lines().map(parse_line::<i32>).collect::<Vec<_>>();

    let now = std::time::Instant::now();
    let result1 = part1(&input, 2_000_000);
    let duration = now.elapsed().as_secs_f64();
    println!("Part 1: {} in {} ms", result1, duration * 1000.0);
    assert_eq!(result1, 4985193);

    // let now = std::time::Instant::now();
    // let result2 = part2_brute_force(&input, 4_000_000);
    // let duration = now.elapsed().as_secs_f64();
    // println!("Part 2 (brute force): {} in {} ms", result2, duration * 1000.0);
    // assert_eq!(result2, 11583882601918);

    let now = std::time::Instant::now();
    let result2 = part2_line_intersect(&input, 4_000_000);
    let duration = now.elapsed().as_secs_f64();
    println!("Part 2 (line intersection): {} in {} ms", result2, duration * 1000.0);
    assert_eq!(result2, 11583882601918);
}

fn part1(pairs: &[(Point<i32>, Point<i32>)], y: i32) -> i32 {
    let mut ranges = RangeSet::new();
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

#[allow(dead_code)]
fn part1_range_set(pairs: &[(Point<i32>, Point<i32>)], y: i32) -> RangeSet<i32> {
    let mut ranges = RangeSet::new();
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

#[allow(dead_code)]
fn part2_brute_force(input: &[(Point<i32>, Point<i32>)], upper_y: i32) -> i64 {
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
            return 4_000_000i64 * (x as i64) + (y as i64);
        }
    }
    panic!("No solution found!");
}

//
// With Manhattan distance, a fixed distance (what would be a circle)
// ends up being a square rotated by 45º relative to the axes.
// The solution to part 2 must be a point that has a distance of 1 more
// than a sensor's radius for at least 4 sensors.
//
// The approach we will use is to find the lines that are at a distance
// of 1 more than any sensor's radius (i.e. just outside the sensor's
// area).  Find all of the points that are an intersection of two such
// lines; these are candidates for the solution.  Of those, find the one
// that is outside the range of all sensors.
//
// Note that all of these lines have a slope of +/- 1.
// We can look for intersections between a slope=1 and slope=-1 line.
//
// The point-slope form of a line is:
//      (y - y1) = m (x - x1) =>
//      y = m(x - x1) + y1
//
// Taking two lines of slope 1 and -1, and solving for x, we get:
//      x = (x2 + y2 + x1 - y1) / 2
// Then substitute into the point-slope equation with m=1 to find y:
//      y = x - x1 + y1
//
// Which points shall we use?  It's convenient to pick points at opposite
// corners of the radius-plus-one square.  Lines with slopes +1 and -1
// passing through those two points form the four lines bounding the
// sensor.  For example, consider a sensor of radius 3:
//
//          x
//         xxx
//        xxxxx
//      PxxxSxxxQ
//        xxxxx
//         xxx
//          x
//
// We'll pick the points marked `P` and `Q`.  To find candidate points, we
// iterate over pairs of points, associate slope 1 with the first point and
// slope -1 with the second point, and find the intersection using the formulas
// above.
//
fn part2_line_intersect(input: &[(Point<i32>, Point<i32>)], upper_y: i32) -> i64 {
    // Build a vector of corner points (P and Q, above)
    let mut corners = Vec::with_capacity(input.len() * 2);
    for (sensor, beacon) in input {
        let dist = sensor.distance_to(beacon) + 1;
        corners.push(Point(sensor.0 - dist, sensor.1));
        corners.push(Point(sensor.0 + dist, sensor.1));
    }

    // Build a vector of candidate solutions
    let mut candidates = HashSet::new();
    for Point(x1,y1) in corners.iter() {
        for Point(x2,y2) in corners.iter() {
            let x = (x2 + y2 + x1 - y1) / 2;
            let y = x - x1 + y1;
            if x >= 0 && x <= upper_y && y >= 0 && y <= upper_y {
                candidates.insert(Point(x,y));
            }
        }
    }

    // Find the one solution
    let Point(x,y) = candidates.iter().find(|p| {
        p.0 >= 0 && p.0 <= upper_y && p.1 >= 0 && p.1 <= upper_y &&
        input.iter().all(|(sensor, beacon)| {
            sensor.distance_to(p) > sensor.distance_to(beacon)
        })
    }).unwrap();
    4_000_000i64 * (*x as i64) + (*y as i64)
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
    let mut right = line.find(',').unwrap();
    let mut x = line[left+2..right].parse::<T>().unwrap();
    line = &line[right..];

    left = line.find("y=").unwrap();
    right = line.find(':').unwrap();
    let mut y = line[left+2..right].parse::<T>().unwrap();
    line = &line[right..];

    let sensor = Point(x,y);

    left = line.find("x=").unwrap();
    right = line.find(',').unwrap();
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
        let input = EXAMPLE.lines().map(parse_line::<i32>).collect::<Vec<_>>();
        assert_eq!(part1(&input, 10), 26);
    }

    #[test]
    fn part2_example() {
        let input = EXAMPLE.lines().map(parse_line::<i32>).collect::<Vec<_>>();
        assert_eq!(part2_brute_force(&input, 20), 56000011);
    }
}
