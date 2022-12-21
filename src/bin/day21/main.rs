use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day21/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 159591692827554);
}

fn part1(input: &str) -> MonkeyNumber {
    let monkeys = parse_input(input);
    monkey_eval("root", &monkeys)
}

fn part2(input: &str) -> MonkeyNumber {
    let _monkeys = parse_input(input);

    // I'm tempted to try changing root's operation to Sub, so that it
    // returns 0 for equality, and try changing humn's value to see how
    // root's value changes.  Hopefully, there is a linear relationship,
    // and I can extrapolate the correct value for humn.

    todo!()
}

fn monkey_eval(name: &str, monkeys: &HashMap<&str, MonkeyJob>) -> MonkeyNumber {
    let result = match monkeys.get(name).unwrap() {
        MonkeyJob::Yell(num) => *num,
        MonkeyJob::Add(left, right) =>
            monkey_eval(left, monkeys) + monkey_eval(right, monkeys),
        MonkeyJob::Sub(left, right) =>
            monkey_eval(left, monkeys) - monkey_eval(right, monkeys),
        MonkeyJob::Mul(left, right) =>
            monkey_eval(left, monkeys) * monkey_eval(right, monkeys),
        MonkeyJob::Div(left, right) =>
            monkey_eval(left, monkeys) / monkey_eval(right, monkeys),
    };
    result
}

fn parse_input(input: &str) -> HashMap<&str, MonkeyJob> {
    let mut monkeys = HashMap::<&str, MonkeyJob>::new();

    for line in input.lines() {
        let (name, job) = line.split_once(": ").unwrap();
        if let Ok(num) = job.parse::<MonkeyNumber>() {
            monkeys.insert(name, MonkeyJob::Yell(num));
        } else {
            let (left, operator, right) = job.split(' ').collect_tuple().unwrap();
            let job = match operator {
                "+" => MonkeyJob::Add(left, right),
                "-" => MonkeyJob::Sub(left, right),
                "*" => MonkeyJob::Mul(left, right),
                "/" => MonkeyJob::Div(left, right),
                _ => panic!("Invalid operation: {}", job)
            };
            monkeys.insert(name, job);
        }
    }

    monkeys
}

type MonkeyNumber = i64;

enum MonkeyJob<'a> {
    Yell(MonkeyNumber),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

#[test]
fn test_part1() {
    let input = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
    assert_eq!(part1(input), 152);
}

#[test]
fn test_part2() {
    let input = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
    assert_eq!(part2(input), 301);
}
