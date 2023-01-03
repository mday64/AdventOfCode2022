use std::collections::HashMap;
use std::time::Instant;
use pathfinding::prelude::{dijkstra,dijkstra_all};

fn main() {
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "src/bin/day16/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let start_time = Instant::now();
    let result1 = part1(&input);
    let duration = start_time.elapsed().as_secs_f64();
    println!("Part 1: {} ({} seconds)", result1, duration);
    assert_eq!(result1, 1701);

    let start_time = Instant::now();
    let result2 = part2b(&input);
    let duration = start_time.elapsed().as_secs_f64();
    println!("Part 2: {} ({} seconds)", result2, duration);
    assert_eq!(result2, 2455);
}

//
// Part 1
//
// The only "interesting" locations are where the valves have non-zero
// flow rates.  Our task is to find a path from valve to valve, turning
// each valve on when we get to it, such that the resulting flow is
// maximized.
//
// Let's use Dijkstra's shortest path algorithm.
//
// But Dijkstra's wants to minimize some cost.  So, what is our "cost"?
// I'm using the "opportunity cost" of unrealized flow from closed valves.
// Cost is the sum of those closed valves' flow rates times elapsed time.
//
// To simplify the number of states we must examine, we need to eliminate
// locations whose flow rate is zero.  But that also means we need to
// calculate the time to move between valves using the locations with
// zero flow.  That is all pairs shortest paths.
//
// Part 1: 1701 (0.323352925 seconds)
fn part1(input: &str) -> i32 {
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    struct State {
        location: String,       // Our current location
        minutes: i32,           // Minutes left
        closed: Vec<String>     // Names of closed valves
    }
    let input = parse_input(input);
    let paths = all_pairs_shortest_paths(&input);
    let valve_names = input.iter().filter_map(|(name, valve)| {
        if valve.flow > 0 {
            Some(name.clone())
        } else {
            None
        }
        }).collect::<Vec<_>>();

    let start = State {
        location: "AA".to_string(),
        minutes: 30,
        closed: valve_names
    };
    let success = |state: &State| state.minutes == 0;
    let successors = |state: &State| -> Vec<(State, i32)> {
        let mut result = Vec::new();
        
        // What is the total flow rate of all closed valves?
        let total_flow: i32 = state.closed.iter()
            .map(|name| input.get(name).unwrap().flow)
            .sum();
        
        // Consider each of the remaining closed valves
        for valve in state.closed.iter().cloned() {
            // Find out how much time to get to that valve and open it
            let time = paths[&(state.location.clone(), valve.clone())] + 1;
            if time < state.minutes {
                let closed = state.closed.iter().filter(|name| **name != valve).cloned().collect();
                result.push((
                    State{ location: valve, minutes: state.minutes-time, closed },
                    time * total_flow
                ));
            }
        }

        // If there wasn't time to close any valves, then the only next state is
        // that we're done.  Don't forget there is a cost!  The actual location
        // and list of closed valves don't matter.
        if result.is_empty() {
            result.push((
                State{ location: String::new(), minutes: 0, closed: vec![] },
                state.minutes * total_flow
            ));
        }
        result
    };

    let (_, cost) = dijkstra(&start, successors, success).unwrap();

    // The answer to part 1 is the total flow that _did_ happen.
    // So that is the maximum possible flow (if all valves had been open
    // at time zero) minus the flow we missed out on while moving from
    // valve to valve.
    let max_flow = input.values().map(|valve| valve.flow * 30).sum::<i32>();
    max_flow - cost
}


//
// The idea here is to see which valves the person could open in the allotted
// time (essentially part 1), then remove those valves from consideration and
// run again for the elephant.  The answer is the total flow from both runs.
//
// Part 2: 2455 (0.13266966 seconds)
fn part2b(input: &str) -> i32 {
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    struct State {
        location: String,
        minutes: i32,
        closed: Vec<String>
    }
    let input = parse_input(input);
    let paths = all_pairs_shortest_paths(&input);
    let valve_names = input.iter().filter_map(|(name, valve)| {
        if valve.flow > 0 {
            Some(name.clone())
        } else {
            None
        }
        }).collect::<Vec<_>>();
    let initial = State { location: "AA".to_string(), minutes: 26, closed: valve_names };
    let success = |state: &State| state.minutes == 0;
    let successors = |state: &State| -> Vec<(State, i32)> {
        let mut result = Vec::new();
        
        // What is the total flow rate of all closed valves?
        let total_flow: i32 = state.closed.iter()
            .map(|name| input.get(name).unwrap().flow)
            .sum();
        
        // Consider each of the remaining closed valves
        for valve in state.closed.iter().cloned() {
            // Find out how much time to get to that valve and open it
            let time = paths[&(state.location.clone(), valve.clone())] + 1;
            if time < state.minutes {
                let closed = state.closed.iter().filter(|name| **name != valve).cloned().collect();
                result.push((
                    State{ location: valve, minutes: state.minutes-time, closed },
                    time * total_flow
                ));
            }
        }

        // If there wasn't time to close any valves, then the only next state is
        // that we're done.  Don't forget there is a cost!  The actual location
        // and list of closed valves don't matter.
        if result.is_empty() {
            result.push((
                State{ location: String::new(), minutes: 0, closed: vec![] },
                state.minutes * total_flow
            ));
        }
        result
    };

    //
    // Let the person do their best to open valves.  Like part 1, with less time.
    //
    let max_flow = input.values().map(|valve| valve.flow * 26).sum::<i32>();
    let (path, cost) = dijkstra(&initial, successors, success).unwrap();
    let person_flow = max_flow - cost;
    
    //
    // Let the elephant open as many of the remaining valves as possible
    //
    let closed_valves = path[path.len() - 2].closed.clone();
    let elephant_max_flow: i32 = input.iter()
        .filter(|(name, _valve)| closed_valves.contains(name))
        .map(|(_name, valve)| valve.flow * 26)
        .sum();
    let initial = State { location: "AA".to_string(), minutes: 26, closed: closed_valves };
    let (_path, elephant_cost) = dijkstra(&initial, successors, success).unwrap();
    let elephant_flow = elephant_max_flow - elephant_cost;

    person_flow + elephant_flow
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
fn test_part1() {
    assert_eq!(part1(EXAMPLE), 1651);
}

#[test]
fn test_part2b() {
    assert_eq!(part2b(EXAMPLE), 1707);
}
