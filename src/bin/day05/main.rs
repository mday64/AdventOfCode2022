fn main() {
    // How much effort do I want to put into parsing the input?  How
    // general do I want it to be?

}

fn part1(input: &str) -> String {
    // How much effort do I want to put into parsing the input?  How
    // general do I want it to be?

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

    // Now process the lines telling us how to move items between stacks

    // Finally, grab the top letter on each stack
    stacks.iter().map(|stack| stack[stack.len()-1]).collect()
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
