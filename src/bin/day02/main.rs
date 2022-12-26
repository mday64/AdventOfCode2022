use anyhow::{anyhow, bail};

fn main() -> anyhow::Result<()> {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day02/input.txt".into());
    let input = std::fs::read_to_string(path)?;

    let result1 = part1(&input)?;
    println!("Part 1: {}", result1);
    assert_eq!(result1, 13268);

    let result2 = part2(&input)?;
    println!("Part 2: {}", result2);
    assert_eq!(result2, 15508);

    Ok(())
}

fn part1(input: &str) -> Result<i32, anyhow::Error> {
    let mut result = 0;
    for line in input.lines() {
        let mut letters = line.split(' ');
        let opponent = letters.next().ok_or_else(|| anyhow!("missing first letter"))?;
        let me = letters.next().ok_or_else(|| anyhow!("missing second letter"))?;

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

fn part2(input: &str) -> Result<i32, anyhow::Error> {
    let mut result = 0;
    for line in input.lines() {
        let mut letters = line.split(' ');
        let opponent = letters.next().ok_or_else(|| anyhow!("missing first letter"))?;
        let outcome = letters.next().ok_or_else(|| anyhow!("missing second letter"))?;

        // Add the score for the outcome of the round
        result += match outcome {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => bail!("Invalid letter for me")
        };

        // Figure out which item I would have to choose, and add its
        // value to the score
        result += match (opponent, outcome) {
            // I lose
            ("A", "X") => 3,
            ("B", "X") => 1,
            ("C", "X") => 2,

            // draw
            ("A", "Y") => 1,
            ("B", "Y") => 2,
            ("C", "Y") => 3,

            // I win
            ("A", "Z") => 2,
            ("B", "Z") => 3,
            ("C", "Z") => 1,

            (_, _) => bail!("Invalid letter combination")
        };
    }
    Ok(result)
}
