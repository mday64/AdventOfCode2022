use std::{fmt::Debug, iter::Sum, ops::{Range, Sub}};

#[derive(Debug)]
pub struct RangeSet<T>
{
    // All of the ranges are non-overlapping.
    // They are in sorted order.
    // No range is empty.
    pub ranges: Vec<Range<T>>
}

impl<T> RangeSet<T>
where T: Copy + Clone + PartialOrd + Ord + Sum<T> + Sub + Sum<<T as Sub>::Output>
{
    pub fn new() -> Self {
        Self { ranges: Vec::new() }
    }

    pub fn insert(&mut self, range: Range<T>) {
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

    pub fn remove(&mut self, removed: Range<T>) {
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

    pub fn intersect(&mut self, keep: Range<T>) {
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

    pub fn len(&self) -> T {
        assert!(self.is_consistent());
        self.ranges.iter().map(|r| r.end - r.start).sum()
    }

    pub fn is_consistent(&self) -> bool {
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

impl<T> Default for RangeSet<T>
where T: Copy + Clone + PartialOrd + Ord + Sum<T> + Sub + Sum<<T as Sub>::Output>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
