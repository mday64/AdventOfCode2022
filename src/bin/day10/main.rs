use std::sync::mpsc::{channel, Sender};

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day10/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    //
    // Part 1
    //
    let result1 = part1(input.clone());
    println!("Part 1: {}", result1);
    assert_eq!(result1, 12460);

    //
    // Part 2
    //
    // Produces the letters: EZFPRAKL
    //
    part2(input);
}

fn part1(input: String) -> i32 {
    let mut result = 0;
    let (tx, rx) = channel();
    std::thread::spawn(|| run_program(input, tx));
    for (i,x) in (1..=220).zip(rx.into_iter()) {
        if i % 40 == 20 {
            result += i * x;
        }
    }
    result
}

fn part2(input: String) {
    let (tx, rx) = channel();
    std::thread::spawn(|| run_program(input, tx));

    for _ in 0..6 {
        for x in 0..40 {
            let xreg = rx.recv().unwrap();
            if (x - xreg).abs() <= 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn run_program(program: String, sender: Sender<i32>) {
    let mut x = 1;
    for line in program.lines() {
        if line == "noop" {
            if sender.send(x).is_err() {
                break;
            }
        } else if line.starts_with("addx ") {
            let (_, v) = line.split_once(' ').unwrap();
            if sender.send(x).is_err() {
                break;
            }
            if sender.send(x).is_err() {
                break;
            }
            x += v.parse::<i32>().unwrap();
        }
    }
}
