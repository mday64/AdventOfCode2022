use std::ops::Range;

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

fn part1(input: &str, y: i64) -> i64 {
    let mut ranges = RangeSet::new();
    let pairs = input.lines().map(parse_line).collect::<Vec<_>>();
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

fn part1_range_set(input: &str, y: i64) -> RangeSet {
    let mut ranges = RangeSet::new();
    let pairs = input.lines().map(parse_line).collect::<Vec<_>>();
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
struct Point(i64, i64);

impl Point {
    fn distance_to(&self, other: &Self) -> i64 {
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
    let mut x = line[left+2..right].parse::<i64>().unwrap();
    line = &line[right..];

    left = line.find("y=").unwrap();
    right = line.find(":").unwrap();
    let mut y = line[left+2..right].parse::<i64>().unwrap();
    line = &line[right..];

    let sensor = Point(x,y);

    left = line.find("x=").unwrap();
    right = line.find(",").unwrap();
    x = line[left+2..right].parse::<i64>().unwrap();
    line = &line[right..];

    left = line.find("y=").unwrap();
    y = line[left+2..].parse::<i64>().unwrap();

    let beacon = Point(x,y);

    (sensor, beacon)
}

#[derive(Debug)]
struct RangeSet {
    // All of the ranges are non-overlapping.
    // They are in sorted order.
    // No range is empty.
    ranges: Vec<Range<i64>>
}

impl RangeSet {
    fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    fn insert(&mut self, range: Range<i64>) {
        //TODO: This could probably be handled by Iterator::scan

        if range.is_empty() {
            return;
        }

        // First, insert the new range in sorted order.
        self.ranges.push(range);
        self.ranges.sort_unstable_by_key(|r| r.start);

        // Now collapse any overlapping or adjacent ranges.
        let mut head = 0;   // Last index of fixed-up ranges
        let mut tail = 1;   // Index of next range to consider
        while tail < self.ranges.len() {
            if self.ranges[head].end < self.ranges[tail].start {
                // Ranges are disjoint
                if head+1 < tail {
                    self.ranges[head+1] = self.ranges[tail].clone();
                }
                head += 1;
                tail += 1;
            } else {
                // Combine these two ranges
                self.ranges[head].end = self.ranges[head].end.max(self.ranges[tail].end);
                tail += 1;
            }
        }

        self.ranges.truncate(head+1);
    }

    fn remove(&mut self, removed: Range<i64>) {
        // We could just adjust start/end of existing ranges, and remove
        // ranges that have become empty.  The one remaining case would
        // be that the input `range` is in the middle of an existing range,
        // which then needs to be replaced with two disjoint ranges.
        
        if removed.is_empty() {
            return;
        }

        self.ranges = self.ranges.iter().flat_map(|r| {
            if r.end <= removed.start {
                return vec![r.clone()];
            }
            if r.start >= removed.end {
                return vec![r.clone()];
            }

            // If we get this far, there is some overlap

            // See if `removed` completely covers `r`
            if removed.start <= r.start && removed.end >= r.end {
                return vec![];
            }
            // See if we need to split `r` into two ranges
            if r.start < removed.start && removed.end < r.end {
                return vec![r.start .. removed.start, removed.end .. r.end];
            }

            // If we get this far, either the start or the end of `r`
            // (but not both) needs to be adjusted.
            let mut res = r.clone();
            if removed.start <= res.start {
                res.start = removed.end;
            } else {
                res.end = removed.start;
            }
            assert!(!res.is_empty());
            vec![res]
        }).collect();
    }

    fn intersect(&mut self, keep: Range<i64>) {
        self.ranges = self.ranges.iter().filter_map(|r| {
            if r.end <= keep.start {
                return None;
            }
            if r.start >= keep.end {
                return None;
            }
            if keep.start <= r.start && keep.end >= r.end {
                return Some(r.clone());
            }

            // If we get here, we trim the head and/or tail of r.
            let mut res = r.clone();
            if keep.start > r.start {
                res.start = keep.start;
            }
            if keep.end < r.end {
                res.end = keep.end;
            }
            Some(res)
        }).collect();
    }

    fn len(&self) -> i64 {
        assert!(self.is_consistent());
        self.ranges.iter().map(|r| r.end - r.start).sum()
    }

    fn is_consistent(&self) -> bool {
        if self.ranges.is_empty() {
            return true;
        }
        let mut ranges = self.ranges.iter();
        let mut previous = ranges.next().unwrap();
        if previous.is_empty() {
            return false;
        }

        for range in ranges {
            if range.is_empty() {
                return false;
            }
            if range.start <= previous.end {
                return false;
            }
            previous = range;
        }
        
        true
    }
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
    fn test_range_set_len() {
        let mut set = RangeSet::new();
        set.insert(3..8);
        assert_eq!(set.len(), 5);
        set.insert(27..42);
        assert_eq!(set.len(), 20);
    }

    #[test]
    fn test_range_set_insert_overlap() {
        let mut set = RangeSet::new();
        set.insert(7..12);
        assert_eq!(set.len(), 5);
        set.insert(3..8);
        assert!(set.is_consistent());
        assert_eq!(set.len(), 9);
    }

    #[test]
    fn test_range_set_insert_adjacent() {
        let mut set = RangeSet::new();
        set.insert(7..12);
        assert_eq!(set.len(), 5);
        set.insert(3..7);
        assert!(set.is_consistent());
        assert_eq!(set.len(), 9);
        assert_eq!(set.ranges, vec![3..12]);
    }


    #[test]
    fn test_range_set_insert_almost_adjacent() {
        let mut set = RangeSet::new();
        set.insert(13..18);
        set.insert(2..6);
        set.insert(7..12);
        assert!(set.is_consistent());
        assert_eq!(set.ranges, vec![2..6, 7..12, 13..18]);
    }

    #[test]
    fn test_range_set_insert_multi_overlap() {
        let mut set = RangeSet::new();
        set.insert(25..29);
        set.insert(7..12);
        set.insert(3..5);
        set.insert(17..23);
        set.insert(11..26);
        assert!(set.is_consistent());
        assert_eq!(set.ranges, vec![3..5, 7..29]);
    }

    #[test]
    fn test_remove_not_found() {
        let mut set = RangeSet::new();
        set.insert(12..25);
        set.remove(3..7);
        set.remove(31..42);
        assert_eq!(set.ranges, vec![12..25]);
    }
    
    #[test]
    fn test_remove_head() {
        let mut set = RangeSet::new();
        set.insert(12..25);
        set.remove(3..17);
        assert_eq!(set.ranges, vec![17..25]);
    }
    
    #[test]
    fn test_remove_tail() {
        let mut set = RangeSet::new();
        set.insert(12..25);
        set.remove(23..37);
        assert_eq!(set.ranges, vec![12..23]);
    }
    
    #[test]
    fn test_remove_middle() {
        let mut set = RangeSet::new();
        set.insert(12..25);
        set.remove(16..19);
        assert_eq!(set.ranges, vec![12..16, 19..25]);
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
