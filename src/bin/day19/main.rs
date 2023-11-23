use pathfinding::prelude::dfs_reach;
use rayon::prelude::*;

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day19/input.txt".into());
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
        blueprint.id * collect_geodes(blueprint, 24)
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

    // Best solution found so far
    let mut max_geodes = 0;

    let collect_resources = |state: &mut State| {
        if state.minutes > 0 {
            state.minutes -= 1;
            state.ore += state.ore_robots;
            state.clay += state.clay_robots;
            state.obsidian += state.obsidian_robots;
            state.geodes += state.geode_robots;

            // Prune distinct states where we have more resources than we could possibly use
            if state.ore_robots >= max_ore_robots && state.ore > max_ore_robots {
                state.ore = max_ore_robots;
            }
            if state.clay_robots >= max_clay_robots && state.clay > max_clay_robots {
                state.clay = max_clay_robots;
            }
            if state.obsidian_robots >= max_obsidian_robots && state.obsidian > max_obsidian_robots {
                state.obsidian = max_obsidian_robots;
            }
        }
    };

    let max_geodes_for_state = |state: &State| -> u16 {
        let mut next_state = state.clone();
        while next_state.minutes > 0 {
            next_state.geodes += next_state.geode_robots;
            next_state.geode_robots += 1;
            next_state.minutes -= 1;
        }
        next_state.geodes
    };

    // Advance state until the requested resources are available.  If so,
    // subtract those resources and return Some(state).  Otherwise, return None.
    let wait_for_resources = |state: &State,
                                ore_cost: u16, clay_cost: u16,
                                obsidian_cost: u16| -> Option<State>
    {
        let mut next_state = state.clone();
        while next_state.minutes > 1 && (next_state.ore < ore_cost || next_state.clay < clay_cost || next_state.obsidian < obsidian_cost) {
            collect_resources(&mut next_state);
        }
        
        if next_state.minutes > 0 && next_state.ore >= ore_cost && next_state.clay >= clay_cost && next_state.obsidian >= obsidian_cost {
            next_state.ore -= ore_cost;
            next_state.clay -= clay_cost;
            next_state.obsidian -= obsidian_cost;
            Some(next_state)
        } else {
            None
        }
    };

    let start = State {
        minutes, ore: 0, clay: 0, obsidian: 0, geodes: 0,
        ore_robots: 1, clay_robots: 0, obsidian_robots: 0, geode_robots: 0,
    };
    let successors = |state: &State| {
        // dbg!(state);
        max_geodes = max_geodes.max(state.geodes);

        let mut result = vec![];

        if state.minutes == 0 {
            return result;
        }

        // If this state couldn't possibly beat our best solution, ignore it.
        if max_geodes_for_state(state) <= max_geodes {
            return result;
        }

        //
        // Pick which robot types we can make, and generate the states
        // after those robots have been constructed.
        //
        
        // Can we make a geode robot?
        if let Some(mut next_state) = wait_for_resources(state, blueprint.geode_robot_ore_cost, 0, blueprint.geode_robot_obsidian_cost) {
            collect_resources(&mut next_state);
            next_state.geode_robots += 1;
            result.push(next_state);
        }

        // Can/should we make an obsidian robot?
        if state.obsidian_robots < max_obsidian_robots {
            if let Some(mut next_state) = wait_for_resources(state, blueprint.obsidian_robot_ore_cost, blueprint.obsidian_robot_clay_cost, 0) {
                collect_resources(&mut next_state);
                next_state.obsidian_robots += 1;
                result.push(next_state);
            }
        }

        // Can/should we make a clay robot?
        if state.clay_robots < max_clay_robots {
            if let Some(mut next_state) = wait_for_resources(state, blueprint.clay_robot_ore_cost, 0, 0) {
                collect_resources(&mut next_state);
                next_state.clay_robots += 1;
                result.push(next_state);
            }
        }

        // Can/should we make an ore robot?
        if state.ore_robots < max_ore_robots {
            if let Some(mut next_state) = wait_for_resources(state, blueprint.ore_robot_ore_cost, 0, 0) {
                collect_resources(&mut next_state);
                next_state.ore_robots += 1;
                result.push(next_state);
            }
        }

        // If we weren't able to make any robots, it's because there isn't
        // enough time left to make anything we need.  So just advance time
        // to the end.
        if result.is_empty() {
            let mut next_state = state.clone();
            while next_state.minutes > 0 {
                collect_resources(&mut next_state);
            }
            result.push(next_state);
        }

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
fn test_collect_geodes() {
    let blueprint = Blueprint::new("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.");
    assert_eq!(collect_geodes(&blueprint, 24), 9);
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
