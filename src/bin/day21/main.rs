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
    let mut monkeys = HashMap::<&str, MonkeyInfo>::new();

    // Parse the input
    for line in input.lines() {
        let (name, job) = line.split_once(": ").unwrap();
        if let Ok(num) = job.parse::<MonkeyNumber>() {
            monkeys.insert(name, MonkeyInfo{cached: Some(num), job: MonkeyJob::Yell(num)});
        } else {
            let (left, operator, right) = job.split(' ').collect_tuple().unwrap();
            let job = match operator {
                "+" => MonkeyJob::Add(left, right),
                "-" => MonkeyJob::Sub(left, right),
                "*" => MonkeyJob::Mul(left, right),
                "/" => MonkeyJob::Div(left, right),
                _ => panic!("Invalid operation: {}", job)
            };
            monkeys.insert(name, MonkeyInfo{cached: None, job});
        }
    }

    monkey_eval("root", &mut monkeys)
}

fn monkey_eval(name: &str, monkeys: &mut HashMap<&str, MonkeyInfo>) -> MonkeyNumber {
    let info = monkeys.get(name).unwrap();
    if let Some(result) = info.cached {
        return result;
    }
    drop(info);
    let result = match info.job {
        MonkeyJob::Yell(num) => num,
        MonkeyJob::Add(left, right) =>
            monkey_eval(left, monkeys) + monkey_eval(right, monkeys),
        MonkeyJob::Sub(left, right) =>
            monkey_eval(left, monkeys) - monkey_eval(right, monkeys),
        MonkeyJob::Mul(left, right) =>
            monkey_eval(left, monkeys) * monkey_eval(right, monkeys),
        MonkeyJob::Div(left, right) =>
            monkey_eval(left, monkeys) / monkey_eval(right, monkeys),
    };
    monkeys.get_mut(name).unwrap().cached = Some(result);
    result
}

type MonkeyNumber = i64;

enum MonkeyJob<'a> {
    Yell(MonkeyNumber),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

struct MonkeyInfo<'a> {
    cached: Option<MonkeyNumber>,
    job: MonkeyJob<'a>
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
