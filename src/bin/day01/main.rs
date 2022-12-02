use anyhow;
use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("src/bin/day01/input.txt")
        .context("opening input")?;

    let mut elf_totals = input.split("\n\n").map(|s| {
        // s is the numbers for one elf
        s.split_terminator('\n').map(|n|
            // n is one number for the current elf
            n.parse::<u32>()
        ).sum()
    }).collect::<Result<Vec<u32>,_>>().context("parsing numbers")?;

    // Part 1
    let result1 = *elf_totals.iter().max().unwrap();
    println!("Part 1: {}", result1);
    assert_eq!(result1, 70374);

    // Part 2
    elf_totals.sort_unstable();
    let result2: u32 = elf_totals.iter().rev().take(3).sum();
    println!("Part 2: {}", result2);
    assert_eq!(result2, 204610);

    Ok(())
}
