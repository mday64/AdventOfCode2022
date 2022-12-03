use std::collections::HashSet;
use itertools::Itertools;
use anyhow::{self, Context};

fn main() -> anyhow::Result<()> {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day03/input.txt".into());
    let input = std::fs::read_to_string(path).context("opening input")?;

    //
    // Part 1
    //
    // For each line of input, we are asked to find which one character
    // appears in both halves of the line.  We then convert that character
    // to a number ("priority").  The final answer is the sum of those
    // numbers ("priorities").
    //
    let result1: u32 = input.lines().map(|line| {
        assert!(line.len() % 2 == 0);
        let mid = line.len() / 2;

        let first_half = line[..mid].chars().collect::<HashSet<char>>();
        let second_half = line[mid..].chars().collect::<HashSet<char>>();
        let overlap = *first_half.intersection(&second_half).next().unwrap();

        match overlap {
            'a'..='z' => 1 + overlap as u32 - 'a' as u32,
            'A'..='Z' => 27 + overlap as u32 - 'A' as u32,
            _ => panic!("Not a letter")
        }
    })
    .sum();
    println!("Part 1: {}", result1);
    assert_eq!(result1, 7872);

    //
    // Part 2
    //
    // This time, we're trying to find the one common letter for every 3 lines.
    // Use the same mechanism to compute "priorities" and sum them.
    //
    let result2: u32 = input.lines().tuples().map(|(line1, line2, line3)| {
        let letters1 = line1.chars().collect::<HashSet<char>>();
        let letters2 = line2.chars().collect::<HashSet<char>>();
        let letters3 = line3.chars().collect::<HashSet<char>>();
        let overlap = letters1.intersection(&letters2).copied().collect::<HashSet<char>>();
        let overlap = *overlap.intersection(&letters3).next().unwrap();

        match overlap {
            'a'..='z' => 1 + overlap as u32 - 'a' as u32,
            'A'..='Z' => 27 + overlap as u32 - 'A' as u32,
            _ => panic!("Not a letter")
        }
    })
    .sum();
    println!("Part 2: {}", result2);
    assert_eq!(result2, 2497);

    Ok(())
}
