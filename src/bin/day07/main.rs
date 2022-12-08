fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day07/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 1555642);
}

fn part1(input: &str) -> u32 {
    //
    // Part 1
    //
    // An interesting puzzle, with a hint of PTSD.  For part 1, I don't
    // think we need to construct the hierarchy, or even keep track of
    // directory or file names.
    //

    let mut totals: Vec<u32> = Vec::new();
    let mut stack:Vec<u32> = Vec::new();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let cd_arg = line.split(' ').skip(2).next().unwrap();
            if cd_arg == ".." {
                // Exiting current directory
                totals.push(stack.pop().unwrap());
            } else {
                // Entering a new directory; don't care about the name
                stack.push(0);
            }
        } else {
            // Try to parse a number at the start of the line
            let word = line.split(' ').next().unwrap();
            if let Ok(v) = word.parse::<u32>() {
                // Add the size of this file to all ancestor directories
                for dir in stack.iter_mut() {
                    *dir += v;
                }
            }
        }
    }

    // Pop any directories still on the stack
    while let Some(v) = stack.pop() {
        totals.push(v);
    }
    
    totals.into_iter().filter(|&v| v <= 100_000).sum()
}

#[cfg(test)]
mod tests {
    use super::part1;
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
        assert_eq!(part1(EXAMPLE_INPUT), 95437);
    }
}