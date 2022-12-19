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
    
    let result2 = part2(&blueprints);
    println!("Part 2: {}", result2);
    assert_eq!(result2, 3510);
}

fn part1(blueprints: &[Blueprint]) -> u16 {
    blueprints.par_iter().map(|blueprint|
        blueprint.id as u16 * collect_geodes(blueprint, 24) as u16
    ).sum()
}

fn part2(blueprints: &[Blueprint]) -> u32 {
    blueprints.par_iter().take(3).map(|blueprint|
        collect_geodes(blueprint, 32) as u32
    ).product()
}

//
// Over very long intervals, since your robot factory can only make one
// robot at a time, you want just enough obsidian robots and ore robots
// to make just enough obsidian and ore to make one geode robot.
// To get to that state, you want just enough ore and clay robots to
// make enough ore and clay to make one obsidian robot.  And so on for
// the number of ore robots to make one clay robot per minute.
//
// All of that means that I don't think we ever need more of a given
// type of robot than the maximum number of that material needed for
// any robot.  I think that is a way to prune possible future states.
//
fn collect_geodes(blueprint: &Blueprint, minutes: u16) -> u16 {
    // Use a depth-first search to find all combinations
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    struct State {
        minutes: u16,
        ore: u16,
        clay: u16,
        obsidian: u16,
        geodes: u16,
        ore_robots: u16,
        clay_robots: u16,
        obsidian_robots: u16,
        geode_robots: u16
    }

    // Precompute the max number of each type of robot
    let max_ore_robots = blueprint.clay_robot_ore_cost
        .max(blueprint.obsidian_robot_ore_cost)
        .max(blueprint.geode_robot_ore_cost);
    let max_clay_robots = blueprint.obsidian_robot_clay_cost;
    let max_obsidian_robots = blueprint.geode_robot_obsidian_cost;
    // There is no max_geode_robots, since we want as many as possible

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
        // The robot factory can only build one robot at a time.
        // Don't forget to add in the materials made by the pre-existing
        // robots.
        if state.ore >= blueprint.geode_robot_ore_cost &&
            state.obsidian >= blueprint.geode_robot_obsidian_cost
        {
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
        if state.obsidian_robots < max_obsidian_robots &&
            state.ore >= blueprint.obsidian_robot_ore_cost &&
            state.clay >= blueprint.obsidian_robot_clay_cost
        {
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
        if state.clay_robots < max_clay_robots &&
            state.ore >= blueprint.clay_robot_ore_cost
        {
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
        if state.ore_robots < max_ore_robots &&
            state.ore >= blueprint.ore_robot_ore_cost
        {
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

        // Try accumulating materials without building a robot
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
        .filter(|state| state.minutes == 0)
        .map(|state| state.geodes)
        .max().unwrap()
}

#[derive(Debug)]
struct Blueprint {
    id: u16,
    ore_robot_ore_cost: u16,
    clay_robot_ore_cost: u16,
    obsidian_robot_ore_cost: u16,
    obsidian_robot_clay_cost: u16,
    geode_robot_ore_cost: u16,
    geode_robot_obsidian_cost: u16,
}

impl Blueprint {
    fn new(line: &str) -> Self {
        let mut numbers = line.split(&[' ', ':'])
            .filter_map(|word| word.parse::<u16>().ok());
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

#[test]
fn test_part2() {
    let input = "\
    Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\n\
    Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.\n\
    ";
    let blueprints: Vec<_> = input.lines().map(Blueprint::new).collect();
    assert_eq!(part2(&blueprints), 56 * 62);
}
