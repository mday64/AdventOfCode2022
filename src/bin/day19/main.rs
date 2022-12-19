use pathfinding::prelude::dfs_reach;
use rayon::prelude::*;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day19/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();
    let blueprints: Vec<_> = input.lines().map(Blueprint::new).collect();
    
    let result1 = part1(&blueprints);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 1199);
}

fn part1(blueprints: &[Blueprint]) -> u32 {
    blueprints.par_iter().map(|blueprint|
        dbg!(blueprint.id) * dbg!(collect_geodes(blueprint, 24))
    ).sum()
}

fn collect_geodes(blueprint: &Blueprint, minutes: u32) -> u32 {
    // Use a depth-first search to find all combinations
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct State {
        minutes: u32,
        ore: u32,
        clay: u32,
        obsidian: u32,
        geodes: u32,
        ore_robots: u32,
        clay_robots: u32,
        obsidian_robots: u32,
        geode_robots: u32
    }

    let start = State {
        minutes, ore: 0, clay: 0, obsidian: 0, geodes: 0,
        ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0,
    };
    let successors = |state: &State| {
        if state.minutes == 0 {
            return vec![];
        }

        let mut result = vec![];
        let minutes = state.minutes - 1;

        // Figure out whether we can start building a new robot.
        // I'm going to assume a max of one new robot, and try them in
        // order from most costly to least costly.
        if state.ore >= blueprint.geode_robot_ore_cost && state.obsidian >= blueprint.geode_robot_obsidian_cost {
            result.push(State {
                minutes,
                ore: state.ore - blueprint.geode_robot_ore_cost + state.ore_robots,
                clay: state.clay + state.clay_robots,
                obsidian: state.obsidian - blueprint.geode_robot_obsidian_cost + state.obsidian_robots,
                geodes: state.geodes + state.geode_robots,
                geode_robots: state.geode_robots+1,
                ..*state
            });
        }
        if state.ore >= blueprint.obsidian_robot_ore_cost && state.clay >= blueprint.obsidian_robot_clay_cost {
            result.push(State {
                minutes,
                ore: state.ore - blueprint.obsidian_robot_ore_cost + state.ore_robots,
                clay: state.clay - blueprint.obsidian_robot_clay_cost + state.clay_robots,
                obsidian: state.obsidian + state.obsidian_robots,
                geodes: state.geodes + state.geode_robots,
                obsidian_robots: state.obsidian_robots+1,
                ..*state
            });
        }
        if state.ore >= blueprint.clay_robot_ore_cost {
            result.push(State {
                minutes,
                ore: state.ore - blueprint.clay_robot_ore_cost + state.ore_robots,
                clay: state.clay + state.clay_robots,
                obsidian: state.obsidian + state.obsidian_robots,
                geodes: state.geodes + state.geode_robots,
                clay_robots: state.clay_robots+1,
                ..*state
            });
        }
        if state.ore >= blueprint.ore_robot_ore_cost {
            result.push(State {
                minutes,
                ore: state.ore - blueprint.ore_robot_ore_cost + state.ore_robots,
                clay: state.clay + state.clay_robots,
                obsidian: state.obsidian + state.obsidian_robots,
                geodes: state.geodes + state.geode_robots,
                ore_robots: state.ore_robots+1,
                ..*state
            });
        }

        // Robots collect their materials
        result.push(State {
            minutes,
            ore: state.ore + state.ore_robots,
            clay: state.clay + state.clay_robots,
            obsidian: state.obsidian + state.obsidian_robots,
            geodes: state.geodes + state.geode_robots, ..*state
        });

        result
    };

    dfs_reach(start, successors)
        .filter_map(|state|
            if state.minutes == 0 {
                Some(state.geodes)
            } else {
                None
            }
        ).max().unwrap()
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

impl Blueprint {
    fn new(line: &str) -> Self {
        let mut numbers = line.split(&[' ', ':'])
            .filter_map(|word| word.parse::<u32>().ok());
        let id = numbers.next().unwrap();
        let ore_robot_ore_cost = numbers.next().unwrap();
        let clay_robot_ore_cost = numbers.next().unwrap();
        let obsidian_robot_ore_cost = numbers.next().unwrap();
        let obsidian_robot_clay_cost = numbers.next().unwrap();
        let geode_robot_ore_cost = numbers.next().unwrap();
        let geode_robot_obsidian_cost = numbers.next().unwrap();
        Self {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        }
    }
}

#[test]
fn test_part1() {
    let input = "\
    Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\n\
    Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.\n\
    ";
    let blueprints: Vec<_> = input.lines().map(Blueprint::new).collect();
    assert_eq!(part1(&blueprints), 33);
}
