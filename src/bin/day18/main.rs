use std::{cell::RefCell, collections::{HashSet,HashMap}, ops::RangeInclusive};
use pathfinding::prelude::dijkstra;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day18/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let now = std::time::Instant::now();
    let result1 = part1(&input);
    let duration = now.elapsed();
    println!("Part 1: {result1} in {duration:?}");
    assert_eq!(result1, 3500);

    let now = std::time::Instant::now();
    let result2 = part2(&input);
    let duration = now.elapsed();
    println!("Part 2: {result2} in {duration:?}");
    assert_eq!(result2, 2048);
}

fn part1(input: &str) -> usize {
    let cubes = parse_input(input);

    cubes.iter().map(|cube| {
        cube_neighbors(cube)
            .iter()
            .filter(|cube| !cubes.contains(cube))
            .count()
    }).sum()
}

//
// Part 2
//
// Similar to part 1, except don't count faces where the neighbor is
// interior to the droplet (i.e. there is no path to outside the drop's
// bounding box).
//
fn part2(input: &str) -> usize {
    let lava = Lava::new(input);

    lava.iter().map(|cube| {
        cube_neighbors(cube)
            .iter()
            .filter(|neighbor| lava.is_exterior_cached(neighbor))
            .count()
    }).sum()
}

type Point = (i8,i8,i8);
type BoundingBox = (RangeInclusive<i8>, RangeInclusive<i8>, RangeInclusive<i8>);

struct Lava {
    cubes: HashSet<Point>,
    bounds: BoundingBox,
    exterior_cache: RefCell<HashMap<Point,bool>>,
}

impl Lava {
    fn new(input: &str) -> Self {
        let cubes = parse_input(input);
        let bounds = get_bounds(&cubes);
        let exterior_cache = RefCell::new(HashMap::new());
        Self { cubes, bounds, exterior_cache }
    }

    #[allow(dead_code)]
    fn contains(&self, point: &Point) -> bool {
        self.cubes.contains(point)
    }

    fn iter(&self) -> impl Iterator<Item=&Point> {
        self.cubes.iter()
    }

    fn is_exterior_cached(&self, point: &Point) -> bool {
        if let Some(result) = self.exterior_cache.borrow().get(point) {
            return *result;
        }

        let result = self.is_exterior(point);
        self.exterior_cache.borrow_mut().insert(*point, result);
        result
    }
    //
    // A cube is exterior if it has a path to a point outside the bounding box
    //
    fn is_exterior(&self, point: &Point) -> bool {
        if self.cubes.contains(point) {
            return false;
        }

        let successors = |cube: &Point| -> Vec<(Point,u8)> {
            cube_neighbors(cube).into_iter()
                .filter(|p| !self.cubes.contains(p))
                .map(|p| (p,1))
                .collect()
        };
        let success = |cube: &Point| -> bool {
            !self.bounds.0.contains(&cube.0) ||
            !self.bounds.1.contains(&cube.1) ||
            !self.bounds.2.contains(&cube.2)
        };
        dijkstra(point, successors, success).is_some()
    }
}

fn get_bounds(lava: &HashSet<Point>) -> BoundingBox {
    // Get one of the cubes from the lava to initialize our ranges.
    let cube = lava.iter().next().unwrap();
    let mut xmin = cube.0;
    let mut xmax = cube.0;
    let mut ymin = cube.1;
    let mut ymax = cube.1;
    let mut zmin = cube.2;
    let mut zmax = cube.2;

    // Expand the ranges to include all of the cubes
    for cube in lava {
        xmin = xmin.min(cube.0);
        xmax = xmax.max(cube.0);
        ymin = ymin.min(cube.1);
        ymax = ymax.max(cube.1);
        zmin = zmin.min(cube.2);
        zmax = zmax.max(cube.2);
    }

    (xmin..=xmax, ymin..=ymax, zmin..=zmax)
}

fn cube_neighbors(&(x,y,z): &Point) -> Vec<Point> {
    vec![
        (x+1,y,z),
        (x-1,y,z),
        (x,y+1,z),
        (x,y-1,z),
        (x,y,z+1),
        (x,y,z-1)
    ]
}

fn parse_input(input: &str) -> HashSet<Point> {
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
    assert_eq!(part1(input), 64);
}

#[test]
fn test_part2() {
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
    assert_eq!(part2(input), 58);
}
