use multiset::HashMultiSet;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day06/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    //
    // Part 1
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
    let chars = input.chars().collect::<Vec<char>>();
    let mut set = HashMultiSet::from_iter(chars[0..4].iter().copied());
    let mut result1 = 0;
    for i in 4..chars.len() {
        if set.distinct_elements().count() == 4 {
            result1 = i;
            break;
        }
        set.insert(chars[i]);
        set.remove(&chars[i-4]);
    };
    println!("Part 1: {}", result1);
    assert_eq!(result1, 0);
}
