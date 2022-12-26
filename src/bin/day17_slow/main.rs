fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day17/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let input = input.trim_end();

    let result1 = part1(input);
    println!("Part 1: {}", result1);
    // assert_eq!(result1, 3161);

    let result2 = part2_slow(input);
    println!("Part 2: {}", result2);
    // assert_eq!(result2, 0);
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

    for _iteration in 0..1_000_000_000_000u64 {
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
fn print_chamber(chamber: &[u16], height: usize, rock: &[u16]) {
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

#[test]
fn test_part1() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part1(input), 3068);
}

#[test]
#[ignore]
fn test_part2_slow() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
    assert_eq!(part2_slow(input), 1514285714288);
}
