use anyhow::{self, Context as _};

fn main() -> anyhow::Result<()> {
    let filename = std::env::args().nth(1)
        .unwrap_or_else(|| String::from("src/bin/day01/input.txt"));
    let input = std::fs::read_to_string(&filename)
        .context("opening input")?;

    let mut elf_totals = input.split("\n\n").map(|s| {
        // s is the lines for one elf
        s.split_terminator('\n').map(str::parse::<u32>).sum()
    }).collect::<Result<Vec<u32>,_>>().context("parsing numbers")?;

    // Since part 2 needs the three largest values, we might as well
    // sort from largest to smallest.  Then part 1, which only needs
    // the largest, can directly grab the first one.  Overall, this
    // should be faster than having part 1 iterate over all the values
    // to find the max, and then having part 2 sort them anyway.
    elf_totals.sort_unstable();
    elf_totals.reverse();

    // Part 1
    let result1 = elf_totals[0];
    println!("Part 1: {}", result1);
    assert_eq!(result1, 70374);

    // Part 2
    let result2: u32 = elf_totals.iter().take(3).sum();
    println!("Part 2: {}", result2);
    assert_eq!(result2, 204610);

    Ok(())
}
