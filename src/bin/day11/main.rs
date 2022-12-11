use std::collections::VecDeque;

fn main() {
    //
    // Trying to parse the input, especially the "operation", would add
    // significantly to the difficulty of this problem.  Instead, I'm
    // going to build them directly in code.
    //
    let mut monkeys = vec![
        Monkey::new(
            &[57, 58],
            &|old| old*19,
            &|worry| if worry % 7 == 0 { 2 } else { 3 }),
        Monkey::new(
            &[66, 52, 59, 79, 94, 73],
            &|old| old+1,
            &|worry| if worry % 19 == 0 { 4 } else { 6 }),
        Monkey::new(
            &[80],
            &|old| old+6,
            &|worry| if worry % 5 == 0 { 7 } else { 5 }),
        Monkey::new(
            &[82, 81, 68, 66, 71, 83, 75, 97],
            &|old| old+5,
            &|worry| if worry % 11 == 0 { 5 } else { 2 }),
        Monkey::new(
            &[55, 52, 67, 70, 69, 94, 90],
            &|old| old*old,
            &|worry| if worry % 17 == 0 { 0 } else { 3 }),
        Monkey::new(
            &[69, 85, 89, 91],
            &|old| old+7,
            &|worry| if worry % 13 == 0 { 1 } else { 7 }),
        Monkey::new(
            &[75, 53, 73, 52, 75],
            &|old| old*7,
            &|worry| if worry % 2 == 0 { 0 } else { 4 }),
        Monkey::new(
            &[94, 60, 79],
            &|old| old+2,
            &|worry| if worry % 3 == 0 { 1 } else { 6 }),
    ];

    //
    // Part 1
    //
    for _ in 0..20 {
        Monkey::one_round(&mut monkeys);
    }
    let mut inspections = monkeys.iter().map(|monkey| monkey.inspected).collect::<Vec<_>>();
    inspections.sort();
    inspections.reverse();
    let result1 = inspections[0] * inspections[1];
    println!("Part 1: {}", result1);
    assert_eq!(result1, 50830);
}

struct Monkey<'a>
{
    items: VecDeque<u32>,
    operation: &'a dyn Fn(u32) -> u32,
    test: &'a dyn Fn(u32) -> usize,
    inspected: u32,
}

impl<'a> Monkey<'a> {
    fn new(items: &[u32], operation: &'a dyn Fn(u32) -> u32, test: &'a dyn Fn(u32) -> usize) -> Self {
        Self { items: VecDeque::from_iter(items.iter().copied()), operation, test, inspected: 0 }
    }

    fn throw(&mut self) -> Option<(u32, usize)> {
        let mut worry = self.items.pop_front()?;
        worry = (self.operation)(worry);
        worry /= 3;
        let destination = (self.test)(worry);
        self.inspected += 1;
        Some((worry, destination))
    }

    fn one_round(monkeys: &mut [Monkey]) {
        for i in 0..monkeys.len() {
            while let Some((worry, destination)) = monkeys[i].throw() {
                monkeys[destination].items.push_back(worry);
            }
        }
    }
}
