use multiset::HashMultiSet;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day06/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let chars = input.chars().collect::<Vec<char>>();

    //
    // Part 1
    //
    let result1 = find_distinct(&chars, 4).unwrap();
    println!("Part 1: {}", result1);
    assert_eq!(result1, 1804);

    //
    // Part 2
    //
    let result2 = find_distinct(&chars, 14).unwrap();
    println!("Part 2: {}", result2);
    assert_eq!(result2, 2508);
}

//
// Wow, there are so many ways you could try to solve this, each with
// their own pros and cons.  There are two aspects to the problem:
//      * getting groups of 4 consecutive characters
//      * seeing if there are duplicates in those 4 characters
// Interestingly, 4 characters is *just* large enough to make a
// brute force pairwise comparison tedious.
//
// Itertools::tuple_windows() is an obvious way to get consecutive
// characters, but then you have a tuple, and it's tedious to find
// duplicates.
//
// You could take slices at increasing offsets, and then collect
// the slice into a HashSet to check for duplicates, and add 3 to
// get the total number of characters processed so far.  Downside
// is that you're creating a lot of tiny sets.
//
// You could improve on the slice implementation by using a VecDeque
// to maintain the last 4 letters.
//
// I think I'm going to go with a multiset (bag).  I could create
// my own from a HashMap (not a HashSet!).  Iterate over the characters
// one at a time, adding the next character and removing offset-4,
// until the multiset contains 4 distinct characters.  Removing the
// offset-4 either requires a Vec<char> (to use a numeric index),
// or two iterators at different positions in the string, or something
// like VecDeque to maintain the last 4 characters.
//
fn find_distinct(chars: &[char], count: usize) -> Option<usize> {
    let mut set = HashMultiSet::from_iter(chars[0..count].iter().copied());
    for i in count..chars.len() {
        if set.distinct_elements().count() == count {
            return Some(i);
        }
        set.insert(chars[i]);
        set.remove(&chars[i-count]);
    };
    None
}
