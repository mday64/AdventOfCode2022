use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("src/bin/day01/input.txt")?;

    let mut elf_totals: Vec<u32> = Vec::new();
    for group in input.split("\n\n") {
        let mut total: u32 = 0;
        for line in group.lines() {
            total += line.parse::<u32>()?;
        }
        elf_totals.push(total);
    }

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
