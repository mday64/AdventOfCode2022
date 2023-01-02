fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day07/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let totals = totals(&input);

    //
    // Part 1
    //
    let result1: u32 = totals.iter().filter(|v| **v <= 100_000).sum();
    println!("Part 1: {}", result1);
    assert_eq!(result1, 1555642);

    //
    // Part 2
    //
    let free_space = 70000000 - totals.last().unwrap();
    let space_needed = 30000000 - free_space;
    let result2 = *totals.iter()
        .filter(|v| **v >= space_needed)
        .min().unwrap();
    println!("Part 2: {}", result2);
    assert_eq!(result2, 5974547);
}

//
// Return the total sizes of all directories.  The size for "/" is last.
//
fn totals(input: &str) -> Vec<u32> {
    let mut totals: Vec<u32> = Vec::new();
    let mut stack:Vec<u32> = Vec::new();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let cd_arg = line.split(' ').nth(2).unwrap();
            if cd_arg == ".." {
                // Exit current directory.  Save its total size.
                let total = stack.pop().unwrap();
                totals.push(total);
                // Add that total size to its parent (if any)
                if let Some(top) = stack.last_mut() {
                    *top += total;
                }
            } else {
                // Entering a new directory; don't care about the name.
                // Initialize the size (so far) to 0.
                stack.push(0);
            }
        } else {
            // Try to parse a number at the start of the line (i.e. a file size)
            let word = line.split(' ').next().unwrap();
            if let Ok(v) = word.parse::<u32>() {
                // Add the size of this file to the current directory
                *stack.last_mut().unwrap() += v;
            }
        }
    }

    // Pop any directories still on the stack
    while let Some(v) = stack.pop() {
        // Add that total size to its parent (if any)
        if let Some(top) = stack.last_mut() {
            *top += v;
        }

        totals.push(v);
    }

    totals
}

#[cfg(test)]
mod tests {
    use super::totals;
    const EXAMPLE_INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_part1() {
        let totals = totals(EXAMPLE_INPUT);
        let result1: u32 = totals.iter().filter(|v| **v <= 100_000).sum();
        assert_eq!(result1, 95437);
    }
}
