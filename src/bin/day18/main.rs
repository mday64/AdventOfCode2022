use std::collections::HashSet;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day18/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 3500);
}

fn part1(input: &str) -> u32 {
    let mut result = 0;
    let cubes = parse_input(&input);
    for cube in cubes.iter() {
        for neighbor in cube_neighbors(cube) {
            if !cubes.contains(&neighbor) {
                result += 1;
            }
        }
    }
    result
}

fn cube_neighbors(&(x,y,z): &(i8,i8,i8)) -> Vec<(i8,i8,i8)> {
    vec![
        (x+1,y,z),
        (x-1,y,z),
        (x,y+1,z),
        (x,y-1,z),
        (x,y,z+1),
        (x,y,z-1)
    ]
}

fn parse_input(input: &str) -> HashSet<(i8,i8,i8)> {
    let mut cubes = HashSet::<(i8,i8,i8)>::new();
    for line in input.lines() {
        let mut numbers = line.split(',').map(|s| s.parse::<i8>().unwrap());
        let point = (numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap());
        cubes.insert(point);
    }
    cubes
}

#[test]
fn test_part1() {
    let input = "\
        2,2,2\n\
        1,2,2\n\
        3,2,2\n\
        2,1,2\n\
        2,3,2\n\
        2,2,1\n\
        2,2,3\n\
        2,2,4\n\
        2,2,6\n\
        1,2,5\n\
        3,2,5\n\
        2,1,5\n\
        2,3,5\n";
    assert_eq!(part1(&input), 64);
}