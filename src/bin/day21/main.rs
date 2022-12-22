use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day21/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 159591692827554);

    let result2 = part2(&input);
    println!("Part 2: {result2}");
    // The accepted answer was 3509819803065
    assert!((3509819803065..=3509819803070).contains(&result2));
}

fn part1(input: &str) -> MonkeyNumber {
    let monkeys = parse_input(input);
    monkey_eval("root", &monkeys)
}

fn part2(input: &str) -> MonkeyNumber {
    let mut monkeys = parse_input(input);

    // I'm tempted to try changing root's operation to Sub, so that it
    // returns 0 for equality, and try changing humn's value to see how
    // root's value changes.  Hopefully, there is a linear relationship,
    // and I can binary search for the correct value for humn.
    //
    // Yes, increasing the value for humn seems to get closer to 0
    // (for both the sample input and my full input).  So I'm going to
    // keep doubling the value until it crosses zero, then binary search
    // to find the exact value.

    let root = monkeys.get_mut("root").unwrap();
    if let MonkeyJob::Add(l, r) = root {
        *root = MonkeyJob::Sub(l, r);
    } else {
        panic!("root should be addition!");
    }

    // Get humn's current value
    let human_val = match monkeys.get("humn").unwrap() {
        MonkeyJob::Yell(v) => *v,
        _ => panic!("humn should be a number (Yell)")
    };

    let mut low = human_val;
    let mut high = human_val * 2;
    let mut low_result = human_eval(low, &mut monkeys);
    let mut high_result = human_eval(high, &mut monkeys);
    while low_result.signum() == high_result.signum() {
        low = high;
        low_result = high_result;
        high *= 2;
        high_result = human_eval(high, &mut monkeys);
    }
    // println!("Somewhere between:");
    // println!("    {} => {}", low, human_eval(low, &mut monkeys));
    // println!("    {} => {}", high, human_eval(high, &mut monkeys));

    let mut guess = (low + high) / 2;
    while low < high {
        guess = (low + high) / 2;
        let guess_result = human_eval(guess, &mut monkeys);
        if guess_result == 0 { break; }
        if guess_result.signum() == low_result.signum() {
            low = guess;
        } else {
            high = guess;
        }
    }
    dbg!(low);
    dbg!(high);
    dbg!(guess);

    // There appear to be multiple valid answers!
    for possible in low ..= high {
        if human_eval(possible, &mut monkeys) == 0 {
            println!("possible: {possible}");
        }
    }
    
    guess
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

fn human_eval(human: MonkeyNumber, monkeys: &mut HashMap<&str, MonkeyJob>) -> MonkeyNumber {
    // Change the value that "humn" yells, and evaluate "root"
    monkeys.insert("humn", MonkeyJob::Yell(human));
    monkey_eval("root", monkeys)
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

#[derive(Clone, Copy)]
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

#[test]
fn test_part2_alt_answer() {
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
    let mut monkeys = parse_input(input);
    monkeys.insert("humn", MonkeyJob::Yell(302));
    let pppw = monkey_eval("pppw", &monkeys);
    let sjmn = monkey_eval("sjmn", &monkeys);
    dbg!(pppw);
    dbg!(sjmn);
    assert_eq!(pppw, sjmn);
}