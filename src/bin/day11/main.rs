use std::collections::VecDeque;

fn main() {
    //
    // Trying to parse the input, especially the "operation", would add
    // significantly to the difficulty of this problem.  Instead, I'm
    // going to build them directly in code.
    //
    let monkeys = vec![
        Monkey::new(
            &[57, 58],
            &|old| old*19,
            7, 2, 3),
        Monkey::new(
            &[66, 52, 59, 79, 94, 73],
            &|old| old+1,
            19, 4, 6),
        Monkey::new(
            &[80],
            &|old| old+6,
            5, 7, 5),
        Monkey::new(
            &[82, 81, 68, 66, 71, 83, 75, 97],
            &|old| old+5,
            11, 5, 2),
        Monkey::new(
            &[55, 52, 67, 70, 69, 94, 90],
            &|old| old*old,
            17, 0, 3),
        Monkey::new(
            &[69, 85, 89, 91],
            &|old| old+7,
            13, 1, 7),
        Monkey::new(
            &[75, 53, 73, 52, 75],
            &|old| old*7,
            2, 0, 4),
        Monkey::new(
            &[94, 60, 79],
            &|old| old+2,
            3, 1, 6),
    ];

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
    let mut inspections = monkeys.iter().map(|monkey| monkey.inspected).collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn part2(mut monkeys: Vec<Monkey>) -> u64 {
    Monkey::many_rounds(&mut monkeys, 10_000, false);
    let mut inspections = monkeys.iter().map(|monkey| monkey.inspected).collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

#[derive(Clone)]
struct Monkey<'a>
{
    items: VecDeque<u64>,
    operation: &'a dyn Fn(u64) -> u64,
    modulo: u64,
    is_divisible: usize,
    not_divisible: usize,
    inspected: u64,
}

impl<'a> Monkey<'a> {
    fn new(items: &[u64], operation: &'a dyn Fn(u64) -> u64, modulo: u64, is_divisible: usize, not_divisible: usize) -> Self {
        Self { items: VecDeque::from_iter(items.iter().copied()), operation, modulo, is_divisible, not_divisible, inspected: 0 }
    }

    fn throw(&mut self, relief: bool, common_modulo: u64) -> Option<(u64, usize)> {
        let mut worry = self.items.pop_front()?;
        worry = (self.operation)(worry);
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
}

#[test]
fn test_rounds_part2() {
    let mut monkeys = vec![
        Monkey::new(&[79, 98], &|old| old*19, 23, 2, 3),
        Monkey::new(&[54, 65, 75, 74], &|old| old+6, 19, 2, 0),
        Monkey::new(&[79, 60, 97], &|old| old*old, 13, 1, 3),
        Monkey::new(&[74], &|old| old+3, 17, 0, 1),
    ];
    Monkey::many_rounds(&mut monkeys, 20, false);
    assert_eq!(monkeys.iter().map(|monkey| monkey.inspected).collect::<Vec<_>>(), vec![99, 97, 8, 103]);
    Monkey::many_rounds(&mut monkeys, 980, false);
    assert_eq!(monkeys.iter().map(|monkey| monkey.inspected).collect::<Vec<_>>(), vec![5204, 4792, 199, 5192]);
    Monkey::many_rounds(&mut monkeys, 9000, false);
    assert_eq!(monkeys.iter().map(|monkey| monkey.inspected).collect::<Vec<_>>(), vec![52166, 47830, 1938, 52013]);
}
