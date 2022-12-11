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
    Monkey::many_rounds(&mut monkeys, 20, true);
    Monkey::monkey_business(&monkeys)
}

fn part2(mut monkeys: Vec<Monkey>) -> u64 {
    Monkey::many_rounds(&mut monkeys, 10_000, false);
    Monkey::monkey_business(&monkeys)
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

    fn throw(&mut self, relief: bool, common_modulo: u64) -> Option<(u64, usize)> {
        let mut worry = self.items.pop_front()?;
        worry = match self.operation {
            Operation::Add(n) => worry + n,
            Operation::Multiply(n) => worry * n,
            Operation::Square => worry * worry,
        };
        if relief {
            worry /= 3;
        } else {
            worry %= common_modulo;
        }
        let destination = if worry % self.modulo == 0 {
            self.is_divisible
        } else {
            self.not_divisible
        };
        self.inspected += 1;
        Some((worry, destination))
    }

    fn one_round(monkeys: &mut [Monkey], relief: bool, common_modulo: u64) {
        for i in 0..monkeys.len() {
            while let Some((worry, destination)) = monkeys[i].throw(relief, common_modulo) {
                monkeys[destination].items.push_back(worry);
            }
        }
    }

    fn many_rounds(monkeys: &mut [Monkey], num_rounds: u32, relief: bool) {
        let common_modulo: u64 = monkeys.iter().map(|monkey| monkey.modulo).product();
        for _ in 0..num_rounds {
            Monkey::one_round(monkeys, relief, common_modulo);
        }
    }

    fn monkey_business(monkeys: &[Monkey]) -> u64 {
        let mut inspections = monkeys
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
    Monkey::many_rounds(&mut monkeys, 20, false);
    assert_eq!(
        monkeys
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>(),
        vec![99, 97, 8, 103]
    );
    Monkey::many_rounds(&mut monkeys, 980, false);
    assert_eq!(
        monkeys
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>(),
        vec![5204, 4792, 199, 5192]
    );
    Monkey::many_rounds(&mut monkeys, 9000, false);
    assert_eq!(
        monkeys
            .iter()
            .map(|monkey| monkey.inspected)
            .collect::<Vec<_>>(),
        vec![52166, 47830, 1938, 52013]
    );
    assert_eq!(Monkey::monkey_business(&monkeys), 2713310158);
}
