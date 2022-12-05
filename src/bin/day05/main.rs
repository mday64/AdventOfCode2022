use std::str::FromStr;
use regex::Regex;
use lazy_static::lazy_static;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day05/input.txt".into());
    let input = std::fs::read_to_string(path)
        .expect("Can't read input");

    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, "PSNRGBTFT");
}

fn part1(input: &str) -> String {
    // Parse the input
    let (mut stacks, movements) = parse_input(input);

    // Now execute the movements
    for Movement{count, source, dest} in movements {
        for _ in 0..count {
            let c = stacks[source-1].pop().unwrap();
            stacks[dest-1].push(c);

            // Note: This doesn't compile:
            // assert_ne!(source, dest);
            // stacks[dest].push(stacks[source].pop().unwrap())
            // because it requires borrowing stacks[] as mutuable twice
        }
    }

    // Finally, grab the top letter on each stack
    stacks.iter().map(|stack| stack[stack.len()-1]).collect()
}

struct Movement {
    count: u32,
    source: usize,
    dest: usize
}

lazy_static! {
    static ref MOVEMENT_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

impl FromStr for Movement {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let caps = MOVEMENT_RE.captures(line).ok_or("Movement syntax error")?;
        let count = caps[1].parse::<u32>().or(Err("Can't parse count"))?;
        let source = caps[2].parse::<usize>().or(Err("Can't parse source"))?;
        let dest = caps[3].parse::<usize>().or(Err("Can't parse dest"))?;

        Ok(Movement { count, source, dest })
    }
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Movement>) {
    // Start by constructing empty stacks.  We examine the length of the
    // first line of input to figure out how many stacks there are.
    // That assumes that those lines are padded with spaces if needed.
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut lines = input.lines();
    let mut line = lines.next().unwrap();
    let num_stacks = (line.len() + 1) / 4;
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    // Now fill in the stacks
    while line.contains('[') {
        let mut chars = line.chars();
        chars.next();       // Skip the leading "["
        for i in 0..num_stacks {
            let c = chars.next().unwrap();
            if c != ' ' {
                stacks[i].push(c);
            }
            chars.next();   // Skip over trailing ']' or space
            chars.next();   // Skip over space between columns
            chars.next();   // Skip over leading '[' or space
        }
        line = lines.next().unwrap();
    }

    // We want the items in the first line to be on the top of the stack,
    // which means we need to reverse each of the stacks.
    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    // Consume the blank line
    assert_eq!(lines.next(), Some(""));

    // Now parse the lines with the movement instructions
    let movements = lines.map(|line| line.parse().unwrap()).collect();

    (stacks, movements)
}

#[cfg(test)]
mod tests {
    use super::part1;
    const EXAMPLE1: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        // Note the [1..] is to skip the initial newline, which is not
        // part of the input.  I can't escape that newline the way I
        // normally would, because it also skips over the leading spaces
        // on the next line.  I need to retain those spaces.  This was
        // the best approach I could come up with that made the example
        // input look like it does on the web page.
        assert_eq!(part1(&EXAMPLE1[1..]), String::from("CMZ"));
    }
}
