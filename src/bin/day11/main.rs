use std::collections::VecDeque;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day11/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let monkeys:Vec<Monkey> = input.split("\n\n").map(Monkey::parse).collect();

    //
    // Part 1
    //
    let result1 = part1(monkeys.clone());
    println!("Part 1: {}", result1);
    assert_eq!(result1, 50830);

    //
    // Part 2
    //
    let result2 = part2(monkeys.clone());
    println!("Part 2: {}", result2);
    assert_eq!(result2, 14399640002);
}

fn part1(mut monkeys: Vec<Monkey>) -> u64 {
    let relax = |worry| worry / 3;
    monkeys.many_rounds(20, relax);
    monkeys.monkey_business()
}

fn part2(mut monkeys: Vec<Monkey>) -> u64 {
    let common_modulo: u64 = monkeys.iter().map(|monkey| monkey.modulo).product();
    let relax = |worry| worry % common_modulo;
    monkeys.many_rounds(10_000, relax);
    monkeys.monkey_business()
}

#[derive(Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn parse(s: &str) -> Self {
        if s == "old * old" {
            Operation::Square
        } else if s.starts_with("old * ") {
            Operation::Multiply(s.rsplit(' ').next().unwrap().parse().unwrap())
        } else if s.starts_with("old + ") {
            Operation::Add(s.rsplit(' ').next().unwrap().parse().unwrap())
        } else {
            panic!("Unknown operation")
        }
    }
}
#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    modulo: u64,
    is_divisible: usize,
    not_divisible: usize,
    inspected: u64,
}

impl Monkey {
    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        lines.next();   // Skip "Monkey n:"
        let (_, items_str) = lines.next().unwrap().split_once(": ").unwrap();
        let items = VecDeque::from_iter(items_str.split(", ").map(|v| v.parse().unwrap()));
        let (_, op_str) = lines.next().unwrap().split_once("new = ").unwrap();
        let operation = Operation::parse(op_str);
        let modulo = lines.next().unwrap().rsplit(' ').next().unwrap().parse::<u64>().unwrap();
        let is_divisible = lines.next().unwrap().rsplit(' ').next().unwrap().parse::<usize>().unwrap();
        let not_divisible = lines.next().unwrap().rsplit(' ').next().unwrap().parse::<usize>().unwrap();
        Self { items, operation: operation, modulo, is_divisible, not_divisible, inspected: 0 }
    }

    fn throw(&mut self, relax: impl Fn(u64)->u64) -> Option<(u64, usize)>
    {
        let mut worry = self.items.pop_front()?;
        worry = match self.operation {
            Operation::Add(n) => worry + n,
            Operation::Multiply(n) => worry * n,
            Operation::Square => worry * worry,
        };
        worry = relax(worry);
        let destination = if worry % self.modulo == 0 {
            self.is_divisible
        } else {
            self.not_divisible
        };
        self.inspected += 1;
        Some((worry, destination))
    }
}

trait MonkeyBusiness {
    fn one_round(&mut self, relax: impl Fn(u64)->u64);
    fn many_rounds(&mut self, num_rounds: u32, relax: impl Fn(u64)->u64);
    fn monkey_business(&self) -> u64;
}
impl MonkeyBusiness for Vec<Monkey> {
    fn one_round(&mut self, relax: impl Fn(u64)->u64) {
        for i in 0..self.len() {
            while let Some((worry, destination)) = self[i].throw(&relax) {
                self[destination].items.push_back(worry);
            }
        }
    }

    fn many_rounds(&mut self, num_rounds: u32, relax: impl Fn(u64)->u64) {
        for _ in 0..num_rounds {
            self.one_round(&relax);
        }
    }

    fn monkey_business(&self) -> u64 {
        let mut inspections = self
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>();
        inspections.sort();
        inspections.reverse();
        inspections[0] * inspections[1]
    }
}

#[test]
fn test_rounds_part2() {
    let input = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    let mut monkeys:Vec<Monkey> = input.split("\n\n").map(Monkey::parse).collect();
    let common_modulo: u64 = monkeys.iter().map(|monkey| monkey.modulo).product();
    let relax = |worry| worry % common_modulo;
    monkeys.many_rounds(20, relax);
    assert_eq!(
        monkeys
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>(),
        vec![99, 97, 8, 103]
    );
    monkeys.many_rounds(980, relax);
    assert_eq!(
        monkeys
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>(),
        vec![5204, 4792, 199, 5192]
    );
    monkeys.many_rounds(9000, relax);
    assert_eq!(
        monkeys
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>(),
        vec![52166, 47830, 1938, 52013]
    );
    assert_eq!(monkeys.monkey_business(), 2713310158);
}
