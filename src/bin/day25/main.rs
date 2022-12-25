fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day25/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {result1}");
    assert_eq!(result1, "2-2--02=1---1200=0-1");
}

fn part1(input: &str) -> String {
    let result = input.lines().map(to_decimal).sum();
    from_decimal(result)
}

fn to_decimal(num: &str) -> i64 {
    let mut result = 0;
    for ch in num.chars() {
        result = result * 5 +
        match ch {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("invalid digit")
        }
    }
    result
}

fn from_decimal(mut num: i64) -> String {
    assert!(num > 0);
    let mut digits = Vec::<i64>::new();
    while num != 0 {
        let mut q = num / 5;
        let mut r = num % 5;
        if r > 2 {
            r -= 5;
            q += 1;
        }
        digits.push(r);
        num = q;
    }

    digits.iter().rev().map(|d| {
        match d {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => panic!("invalid digit")
        }
    }).collect::<String>()
}

#[test]
fn test_to_decimal() {
    assert_eq!(to_decimal("1=-0-2"), 1747);
    assert_eq!(to_decimal("12111"), 906);
    assert_eq!(to_decimal("2=01"), 201);
    assert_eq!(to_decimal("1=-1="), 353);
}

#[test]
fn test_from_decimal() {
    assert_eq!(from_decimal(1), "1");
    assert_eq!(from_decimal(2), "2");
    assert_eq!(from_decimal(3), "1=");
    assert_eq!(from_decimal(4), "1-");
    assert_eq!(from_decimal(5), "10");
    assert_eq!(from_decimal(6), "11");
    assert_eq!(from_decimal(7), "12");
    assert_eq!(from_decimal(8), "2=");
    assert_eq!(from_decimal(9), "2-");
    assert_eq!(from_decimal(10), "20");
    assert_eq!(from_decimal(15), "1=0");
    assert_eq!(from_decimal(20), "1-0");
    assert_eq!(from_decimal(2022), "1=11-2");
    assert_eq!(from_decimal(12345), "1-0---0");
    assert_eq!(from_decimal(314159265), "1121-1110-1=0");
}

#[test]
fn test_part1() {
    let input = "\
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";
    assert_eq!(part1(input), "2=-1=0");
}
