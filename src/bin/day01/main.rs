use std::{ops::AddAssign, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("src/bin/day01/input.txt")?;

    let mut elf_totals = input.split("\n\n").map(|s| {
        // s is the numbers for one elf
        s.split_terminator('\n').map(|n|
            // n is one number for the current elf
            n.parse::<u32>()
        ).try_sum()
    }).collect::<Result<Vec<u32>,_>>()?;

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

fn try_sum<I,T,E>(iter: I) -> Result<T,E>
    where T: Default + AddAssign, I: Iterator<Item=Result<T,E>>
{
    let mut result = T::default();
    for x in iter {
        result += x?;
    }
    Ok(result)
}

trait TrySum: Iterator {
    fn try_sum<T,E>(&mut self) -> Result<T,E>
    where   Self: Iterator<Item=Result<T,E>>,
            T: Default + AddAssign,
    {
        let mut answer = T::default();
        for x in self {
            answer += x?;
        }
        Ok(answer)
    }
}

impl<T> TrySum for T where T: Iterator {}
