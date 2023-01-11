use std::collections::HashMap;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day17/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let input = input.trim_end();

    let now = std::time::Instant::now();
    let result1 = part1(input);
    let duration = now.elapsed().as_secs_f64();
    println!("Part 1: {} (in {} seconds)", result1, duration);
    assert_eq!(result1, 3161);

    let now = std::time::Instant::now();
    let result2 = part2(input);
    let duration = now.elapsed().as_secs_f64();
    println!("Part 2: {} (in {} seconds)", result2, duration);
    assert_eq!(result2, 1575931232076);
}

//
// Part 1
//
// I think the representation for a rock should be one u16 per row.
// The u16 is a bitmap of the positions occupied by the rock.
// Bits 0 and 8 are the chamber walls. Should bits be high to low or
// low to high when going left to right?  For visual convenience, I'll
// go high order to the left. The initial bitmap for a rock is its
// position two units from the left edge.
//
// The chamber where rocks fall will just be a Vec of u16.  Since
// Vec grows most easily from the tail end, I'm going to pretend that
// index 0 is just above the floor, and larger indices are higher up.
// This means that first byte of a rock should be closest to the floor.
// For ease of collision detection, the bitmaps in the chamber will
// have the chamber walls set.
//
fn part1(input: &str) -> usize {
    let mut input = input.chars().cycle();
    let rocks: Vec<Vec<u16>> = vec![
        vec![0b00111100],
        vec![0b00010000, 0b00111000, 0b00010000],
        vec![0b00111000, 0b00001000, 0b00001000],
        vec![0b00100000, 0b00100000, 0b00100000, 0b00100000],
        vec![0b00110000, 0b00110000]
    ];
    let mut rocks = rocks.iter().cycle();

    const CHAMBER_WALLS: u16 = 0b100000001;
    let mut chamber: Vec<u16> = Vec::with_capacity(4000);
    let mut chamber_used = 0;

    for _iteration in 0..2022 {
        // println!("== {_iteration} ==");

        // Get the next rock
        let mut rock = rocks.next().unwrap().clone();

        // Set the initial height of the rock
        let mut height = chamber_used + 3;

        // Make sure the chamber is tall enough to accomodate the
        // current rock at its initial height
        chamber.resize(height + rock.len(), CHAMBER_WALLS);

        // print_chamber(&chamber, height, &rock);
        // std::thread::sleep(std::time::Duration::from_millis(500));

        loop {
            // Try to push rock left or right based on input
            let movement = match input.next().unwrap() {
                '>' => |v: u16| v >> 1,
                '<' => |v: u16| v << 1,
                _ => panic!("invalid input"),
            };
            if rock.iter().enumerate().all(|(i,v)| chamber[height+i] & movement(*v) == 0) {
                for v in rock.iter_mut() {
                    *v = movement(*v);
                }
            }
            // print_chamber(&chamber, height, &rock);
            // std::thread::sleep(std::time::Duration::from_millis(500));

            // Try to push rock down
            if height > 0 && rock.iter().enumerate().all(|(i,v)| chamber[height+i-1] & v == 0) {
                height -= 1;
            } else {
                // Rock comes to rest
                chamber_used = chamber_used.max(height + rock.len());
                for (i, v) in rock.into_iter().enumerate() {
                    chamber[height + i] |= v;
                }
                break;
            }
            // print_chamber(&chamber, height, &rock);
            // std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    chamber_used
}

//
// Clearly, we have to find a repeating pattern, and use that to skip most of
// the repetitions.  A challenge in finding such a pattern is "uneven edges."
// It will consume some initial part of the input before it falls into the
// pattern.  But the most recent (top) rows of the chamber won't match
// because they are not fully filled in yet (they're waiting for more input).
// So how do I correlate position in the input to row position in the chamber?
//
// The cycle detection needs to include the offset into the input (left/right
// moves) sequence, the offset into the rock sequence, and something about
// the shape of the settled rocks.
//
// Actually, can I assume that it will repeat every (# rocks) * (# left/right)
// iterations?  (In my case, 10,091 * 5 = 50,455)
// NO!  A single rock dropping consumes more than one character of input.
//
// For my input, the cycle appears to be 1745 rocks, with a height change
// of 2750.
//
// Note that the cycle is NOT the first time a rock index and jet index repeat.
// It's not even the first time that we see the same change in iterations plus
// change in height.  It appears to be sufficient to find the third occurrence
// of a given (rock_index, jet_index), where the cycle is between the second and
// third occurrences.  Or should I also look at the top of the chamber?  Or look
// for some other quantity to repeat?
//
fn part2(input: &str) -> usize {
    let input_length = input.len();
    let mut input = input.chars().cycle();
    let rocks: Vec<Vec<u16>> = vec![
        vec![0b00111100],
        vec![0b00010000, 0b00111000, 0b00010000],
        vec![0b00111000, 0b00001000, 0b00001000],
        vec![0b00100000, 0b00100000, 0b00100000, 0b00100000],
        vec![0b00110000, 0b00110000]
    ];
    let mut rocks = rocks.iter().cycle();

    const CHAMBER_WALLS: u16 = 0b100000001;
    let mut chamber: Vec<u16> = Vec::with_capacity(4000);
    let mut chamber_used = 0;
    let mut jet_index = 0;

    let mut heights = HashMap::<(u64,usize,Vec<u16>), (u64,usize)>::new();
    let mut cycle_height = 0;
    let mut num_cycles = 0;

    let mut iteration: u64 = 0;
    while iteration < 1_000_000_000_000 {
        // Get the next rock
        let mut rock = rocks.next().unwrap().clone();

        // Set the initial height of the rock
        let mut height = chamber_used + 3;

        // Make sure the chamber is tall enough to accomodate the
        // current rock at its initial height
        chamber.resize(height + rock.len(), CHAMBER_WALLS);

        let live_edge_depth = max_depth(&chamber);
        let live_edge = Vec::from(&chamber[chamber.len() - live_edge_depth..]);

        let rock_index = iteration % 5;     // We have 5 rocks
        if let Some((i,h)) = heights.get(&(rock_index, jet_index, live_edge.clone())) {
            // We have detected a cycle of (iteration-i) rocks, with a height
            // of (chamber_used-h).
            let cycle_length = iteration - i;
            num_cycles = (1_000_000_000_000 - iteration) / cycle_length;
            cycle_height = chamber_used - h;
            iteration += num_cycles * cycle_length;
            heights.clear();
        }
        heights.insert((rock_index, jet_index, live_edge), (iteration,chamber_used));

        loop {
            // Try to push rock left or right based on input
            let movement = match input.next().unwrap() {
                '>' => |v: u16| v >> 1,
                '<' => |v: u16| v << 1,
                _ => panic!("invalid input"),
            };
            jet_index = (jet_index + 1) % input_length;

            if rock.iter().enumerate().all(|(i,v)| chamber[height+i] & movement(*v) == 0) {
                for v in rock.iter_mut() {
                    *v = movement(*v);
                }
            }

            // Try to push rock down
            if height > 0 && rock.iter().enumerate().all(|(i,v)| chamber[height+i-1] & v == 0) {
                height -= 1;
            } else {
                // Rock comes to rest
                chamber_used = chamber_used.max(height + rock.len());
                for (i, v) in rock.into_iter().enumerate() {
                    chamber[height + i] |= v;
                }
                break;
            }
        }
        iteration += 1;
    }

    chamber_used + (num_cycles as usize) * cycle_height
}

//
// Return the maximum depth from the top most rock piece to the deepest
// empty space in any column.  This limits how far the vertical line
// could drop, and should be a good bound for how much of the top of
// the chamber we need to remember in order to detect a repeating pattern.
//
fn max_depth(chamber: &[u16]) -> usize {
    let mut columns = 0;

    for (i,v) in chamber.iter().rev().map(|v| v & 0b011111110).enumerate() {
        columns |= v;
        if columns == 0b011111110 {
            return i+1;
        }
    }
    chamber.len()
}


#[cfg(test)]
fn part2_slow(input: &str) -> usize {
    let mut input = input.chars().cycle();
    let rocks: Vec<Vec<u16>> = vec![
        vec![0b00111100],
        vec![0b00010000, 0b00111000, 0b00010000],
        vec![0b00111000, 0b00001000, 0b00001000],
        vec![0b00100000, 0b00100000, 0b00100000, 0b00100000],
        vec![0b00110000, 0b00110000]
    ];
    let mut rocks = rocks.iter().cycle();

    const CHAMBER_WALLS: u16 = 0b100000001;
    let mut chamber: Vec<u16> = Vec::with_capacity(4000);
    let mut chamber_used = 0;
    let mut shifted = 0;

    for _iteration in 0..2022 {
        if _iteration % 1_000_000 == 0 {
            println!("{}", _iteration/1_000_000);
        }

        // Get the next rock
        let mut rock = rocks.next().unwrap().clone();

        // Set the initial height of the rock
        let mut height = chamber_used + 3;

        // Make sure the chamber is tall enough to accomodate the
        // current rock at its initial height
        chamber.resize(height + rock.len(), CHAMBER_WALLS);

        loop {
            // Try to push rock left or right based on input
            let movement = match input.next().unwrap() {
                '>' => |v: u16| v >> 1,
                '<' => |v: u16| v << 1,
                _ => panic!("invalid input"),
            };
            if rock.iter().enumerate().all(|(i,v)| chamber[height+i] & movement(*v) == 0) {
                for v in rock.iter_mut() {
                    *v = movement(*v);
                }
            }

            // Try to push rock down
            if height > 0 && rock.iter().enumerate().all(|(i,v)| chamber[height+i-1] & v == 0) {
                height -= 1;
            } else {
                // Rock comes to rest
                chamber_used = chamber_used.max(height + rock.len());
                for (i, v) in rock.into_iter().enumerate() {
                    chamber[height + i] |= v;
                }
                break;
            }
        }

        if chamber_used > 1000 {
            // Remove the first 500 items in chamber[]
            chamber.drain(0..900);
            shifted += 900;
            chamber_used -= 900;
        }
    }

    shifted + chamber_used
}

#[allow(dead_code)]
fn print_chamber_and_rock(chamber: &[u16], height: usize, rock: &[u16]) {
    for (h,v) in chamber.iter().enumerate().rev() {
        let mut bits = *v;
        if h >= height && h < (height + rock.len()) {
            bits |= rock[h-height];
        }
        print!("|");
        for mask in [128, 64, 32, 16, 8, 4, 2] {
            if mask & bits != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("---------");
    println!();
}

#[allow(dead_code)]
fn print_chamber(chamber: &[u16]) {
    for v in chamber.iter().rev() {
        let bits = *v;
        print!("|");
        for mask in [128, 64, 32, 16, 8, 4, 2] {
            if mask & bits != 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
    println!("---------");
    println!();
}

#[test]
fn test_part1() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(input), 3068);
}

#[test]
fn test_part2() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part2(input), 1514285714288);
}

#[test]
#[ignore]
fn test_part2_slow() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part2_slow(input), 1514285714288);
}
