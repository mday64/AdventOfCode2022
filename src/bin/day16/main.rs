use std::collections::HashMap;
use pathfinding::prelude::dijkstra_all;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day16/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 1701);

    let result2 = part2(&input);
    println!("Part 2: {}", result2);
    assert_eq!(result2, 0);
}

//
// Part 1
//
// Let's use Dijkstra's shortest path algorithm.  Let's hope that
// pathfinding's algorithm works with negative numbers since we want
// a maximum and the algorithm finds a minimum.  Therefore, we'll use
// negative costs.
//
// Idea: Find the shortest paths between every pair of valves.
// Then, all we have to examine are moves between valves with non-zero
// flow rates (and which have not yet been opened).  A single move is
// go to a location with a closed valve and open it.
//
fn part1(input: &str) -> i32 {
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    struct State {
        location: String,
        minutes: i32,
        closed: Vec<String>
    }
    let input = parse_input(&input);
    let paths = all_pairs_shortest_paths(&input);
    let valve_names = input.iter().filter_map(|(name, valve)| {
        if valve.flow > 0 {
            Some(name.clone())
        } else {
            None
        }
        }).collect::<Vec<_>>();
    let initial = State { location: "AA".to_string(), minutes: 30, closed: valve_names.clone() };
    let successors = |state: &State| -> Vec<(State, i32)> {
        let mut result = Vec::new();
        // Consider each of the remaining closed valves
        for valve in state.closed.iter().cloned() {
            // Find out how much time to get to that valve and open it
            let time = paths[&(state.location.clone(), valve.clone())] + 1;
            if time < state.minutes {
                let cost = -(state.minutes - time)*input[&valve].flow;
                let closed = state.closed.iter().filter(|name| **name != valve).cloned().collect();
                result.push((
                    State{ location: valve, minutes: state.minutes-time, closed },
                    cost
                ));
            }
        }
        result
    };

    // Because "success" is defined as running out of time, we need to call
    // dijkstra_all to examine all possibilities.  Then find the best flow
    // from all of those.
    let result = dijkstra_all(&initial, successors);
    let result = result.values().map(|(state, cost)| (state, -cost)).max_by_key(|(_state, cost)| *cost).unwrap();
    // dbg!(result.0);
    result.1
}

//
// Part 2
//
// Similar to part 1, except that you have two people traveling through
// the graph and turning on valves at the same time.
//
// Here's my idea.  Each person is traveling to some destination.
// The state contains that destination and time remaining to reach it.
// When they reach it (time remaining is zero), they consider every
// valve left to handle that they can reach in the time remaining.
// If both are traveling, then advance time to the soonest point where
// at least one reaches their destination.
//
// Need to handle the possibility that all valves have been opened before
// the total time has elapsed.  Or that one person doesn't have time
// to reach any more valves, but the other does.
//
fn part2(input: &str) -> i32 {
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    struct State {
        person_destination: String,
        person_travel_time: i32,
        elephant_destination: String,
        elephant_travel_time: i32,
        minutes: i32,
        todo: Vec<String>       // Closed valves not yet handled
    }

    let input = parse_input(&input);
    let pairs = all_pairs_shortest_paths(&input);
    let todo = input.iter().filter_map(|(name, valve)| {
        if valve.flow > 0 {
            Some(name.clone())
        } else {
            None
        }
        }).collect::<Vec<_>>();
    let initial = State {
        person_destination: "AA".to_string(),
        person_travel_time: 0,
        elephant_destination: "AA".to_string(),
        elephant_travel_time: 0,
        minutes: 26,
        todo
    };
    let successors = |state: &State| -> Vec<(State, i32)> {
        let mut result = vec![];

        if state.todo.len() == 0 || state.minutes <= 0 {
            // No more valves to consider, so we're done
            return result;
        }

        if state.person_travel_time == 0 {
            // See if the person can reach a valve in the time remaining
            for valve in state.todo.iter() {
                let time_to_valve = pairs[&(state.person_destination.clone(), valve.clone())];
                if time_to_valve + 1 < state.minutes {
                    let mut new_state = state.clone();
                    new_state.person_destination = valve.clone();
                    new_state.person_travel_time = time_to_valve + 1;
                    new_state.todo = state.todo.iter().filter(|name| *name != valve).cloned().collect();
                    let cost = -(state.minutes - time_to_valve - 1) * input[valve].flow;
                    result.push((new_state, cost));
                }
            }
            if result.len() > 0 {
                return result;
            }
        }

        if state.elephant_travel_time == 0 {
            // See if the elephant can reach a valve in the time remaining
            for valve in state.todo.iter() {
                let time_to_valve = pairs[&(state.elephant_destination.clone(), valve.clone())];
                if time_to_valve + 1 < state.minutes {
                    let mut new_state = state.clone();
                    new_state.elephant_destination = valve.clone();
                    new_state.elephant_travel_time = time_to_valve + 1;
                    new_state.todo = state.todo.iter().filter(|name| *name != valve).cloned().collect();
                    let cost = -(state.minutes - time_to_valve - 1) * input[valve].flow;
                    result.push((new_state, cost));
                }
            }
            if result.len() > 0 {
                return result;
            }
        }

        let min_travel_time = state.person_travel_time.min(state.elephant_travel_time);
        if min_travel_time > 0 {
            // Let time pass
            let mut new_state = state.clone();
            new_state.person_travel_time -= min_travel_time;
            new_state.elephant_travel_time -= min_travel_time;
            new_state.minutes -= min_travel_time;
            result.push((new_state, 0));
        }
        result
    };
    
    // Because "success" is defined as running out of time, we need to call
    // dijkstra_all to examine all possibilities.  Then find the best flow
    // from all of those.
    let result = dijkstra_all(&initial, successors);
    let result = result.values().map(|(state, cost)| (state, -cost)).max_by_key(|(_state, cost)| *cost).unwrap();
    // dbg!(result.0);
    result.1
}

fn all_pairs_shortest_paths(input: &HashMap<String, Valve>) -> HashMap<(String, String), i32> {
    let mut result = HashMap::new();

    // We're going to do this the expensive way: via dijkstra_all
    for source in input.keys() {
        let successors = |node: &String| -> Vec<(String, i32)> {
            input[node].neighbors.iter().map(|name| (name.clone(), 1)).collect()
        };
        let paths = dijkstra_all(source, successors);
        for (destination, (_, cost)) in paths.iter() {
            result.insert((source.clone(), destination.clone()), *cost);
        }
    }

    result
}

#[derive(Debug)]
struct Valve {
    flow: i32,
    neighbors: Vec<String>
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    let mut result = HashMap::new();

    for line in input.lines() {
        // Get this valve's name.  All valve names are two characters,
        // so I can slice the input to get the name
        let name = line[6..8].to_string();

        // Get this valve's flow rate.  The flow rate starts at a fixed
        // column.  Find the semicolon and parse that range.
        let semicolon = line.find(';').unwrap();
        let flow = line[23..semicolon].parse().unwrap();

        // Get the names of neighbor valves.  Darned input has both
        // "lead to valve " and "lead to valves ".
        let mut neighbor_offset = line.find("to valve").unwrap() + 8;
        if line[neighbor_offset..].starts_with('s') {
            neighbor_offset += 2;   // Skip over "s "
        } else {
            neighbor_offset += 1;   // Skip over " "
        }
        let neighbors = line[neighbor_offset..]
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        let valve = Valve{flow, neighbors};
        // println!("{name}: {valve:?}");
        result.insert(name, valve);
    }
    result
}

#[cfg(test)]
const EXAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

#[test]
fn test_part1b() {
    assert_eq!(part1(EXAMPLE), 1651);
}

#[test]
fn test_part2() {
    assert_eq!(part2(EXAMPLE), 1707);
}
