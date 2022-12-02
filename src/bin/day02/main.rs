use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day02/input.txt".into());
    let input = std::fs::read_to_string(path)?;

    let result1 = part1(input)?;
    println!("Part 1: {}", result1);
    assert_eq!(result1, 13268);

    Ok(())
}

fn part1(input: String) -> Result<i32, anyhow::Error> {
    let mut result = 0;
    for line in input.lines() {
        let mut letters = line.split(' ');
        let opponent = letters.next().ok_or(anyhow!("missing first letter"))?;
        let me = letters.next().ok_or(anyhow!("missing first letter"))?;

        // Add the score for the item I chose
        result += match me {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => bail!("Invalid letter for me")
        };

        // Add the score for the outcome of the match
        result += match (opponent, me) {
            ("A", "X") | ("B", "Y") | ("C", "Z") => 3,
            ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
            ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
            (_, _) => bail!("Invalid letter combination")
        };
    }
    Ok(result)
}
