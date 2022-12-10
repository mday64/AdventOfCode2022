fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day10/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    //
    // Part 1
    //
    // How should we solve this one?  One of the potential issues I
    // see is that the `addx V` instruction takes two cycles, and the
    // cycles we care about could be in the middle of an instruction.
    // We would need to be able to detect that an interesting cycle
    // happened, and know the value in `X` at the time of the interesting
    // cycle.
    //
    // Another possibility is to create an iterator that produces the
    // value of `X` on every cycle.  Then we skip(), step(), and take()
    // to get the interesting values.
    //
    let mut interesting_cycles = [20, 60, 100, 140, 180, 220].into_iter();
    let mut next_interesting_cycle = interesting_cycles.next().unwrap();
    let mut cycle = 0;
    let mut xreg = 1;
    let mut result1 = 0;
    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
        } else if line.starts_with("addx ") {
            cycle += 2;
        }

        if cycle >= next_interesting_cycle {
            result1 += next_interesting_cycle * xreg;
            if let Some(n) = interesting_cycles.next() {
                next_interesting_cycle = n;
            } else {
                break;
            }
        }

        if line.starts_with("addx ") {
            let (_, amount) = line.split_once(' ').unwrap();
            xreg += amount.parse::<i32>().unwrap();
        }
    }
    println!("Part 1: {}", result1);
    assert_eq!(result1, 12460);
}
