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
// Part 1: 1701 (0.035187718 seconds)
fn part1(input: &str) -> u32 {
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    struct State {
        location: ValveID,       // Our current location
        minutes: u32,           // Minutes left
        closed: u64,            // Bitmap of closed valves
    }
    let (aa_id, valves) = parse_input(input);
    let paths = all_pairs_shortest_paths(&valves);
    let closed = valves.iter().filter_map(|(id, valve)| {
        if valve.flow > 0 {
            Some(1u64 << id)
        } else {
            None
        }
    }).sum();
    let start = State { location: aa_id, minutes: 30, closed };
    let success = |state: &State| state.minutes == 0;
    let successors = |state: &State| -> Vec<(State, u32)> {
        let mut result = Vec::new();
        
        // What is the total flow rate of all closed valves?
        let total_flow: u32 = (0..64).into_iter().map(|id| {
            if state.closed & (1 << id) != 0 {
                valves[&id].flow
            } else {
                0
            }
        }).sum();

        // Consider each of the remaining closed valves
        for id in 0..64 {
            if state.closed & (1 << id) != 0 {
                // Find out how much time to get to that valve and open it
                let time = paths[&(state.location, id)] + 1;
                if time < state.minutes {
                    let closed = state.closed & !(1 << id); // Open valve #`id`
                    result.push((
                        State{ location: id, minutes: state.minutes-time, closed },
                        time * total_flow
                    ));
                }
            }
        }

        // If there wasn't time to close any valves, then the only next state is
        // that we're done.  Don't forget there is a cost!  The actual location
        // and list of closed valves don't matter.
        if result.is_empty() {
            result.push((
                State{ location: 0, minutes: 0, closed: 0 },
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
    let max_flow = valves.values().map(|valve| valve.flow * 30).sum::<u32>();
    max_flow - cost
}


//
// The idea here is to see which valves the person could open in the allotted
// time (essentially part 1), then remove those valves from consideration and
// run again for the elephant.  The answer is the total flow from both runs.
//
// Part 2: 2455 (0.012084316 seconds)
fn part2b(input: &str) -> u32 {
    #[derive(PartialEq, Eq, Hash, Clone, Debug)]
    struct State {
        location: ValveID,
        minutes: u32,
        closed: u64
    }
    let (aa_id, valves) = parse_input(input);
    let paths = all_pairs_shortest_paths(&valves);
    let closed = valves.iter().filter_map(|(id, valve)| {
        if valve.flow > 0 {
            Some(1u64 << id)
        } else {
            None
        }
    }).sum();
    let initial = State { location: aa_id, minutes: 26, closed };
    let success = |state: &State| state.minutes == 0;
    let successors = |state: &State| -> Vec<(State, u32)> {
        let mut result = Vec::new();
        
        // What is the total flow rate of all closed valves?
        let total_flow: u32 = (0..64).into_iter().map(|id| {
            if state.closed & (1 << id) != 0 {
                valves[&id].flow
            } else {
                0
            }
        }).sum();

        // Consider each of the remaining closed valves
        for id in 0..64 {
            if state.closed & (1 << id) != 0 {
                // Find out how much time to get to that valve and open it
                let time = paths[&(state.location, id)] + 1;
                if time < state.minutes {
                    let closed = state.closed & !(1 << id); // Open valve #`id`
                    result.push((
                        State{ location: id, minutes: state.minutes-time, closed },
                        time * total_flow
                    ));
                }
            }
        }

        // If there wasn't time to close any valves, then the only next state is
        // that we're done.  Don't forget there is a cost!  The actual location
        // and list of closed valves don't matter.
        if result.is_empty() {
            result.push((
                State{ location: 0, minutes: 0, closed: 0 },
                state.minutes * total_flow
            ));
        }
        result
    };

    //
    // Let the person do their best to open valves.  Like part 1, with less time.
    //
    let max_flow = valves.values().map(|valve| valve.flow * 26).sum::<u32>();
    let (path, cost) = dijkstra(&initial, successors, success).unwrap();
    let person_flow = max_flow - cost;
    
    //
    // Let the elephant open as many of the remaining valves as possible
    //
    let closed_valves = path[path.len() - 2].closed;
    let elephant_max_flow: u32 = valves.iter()
        .filter(|(id, _valve)| closed_valves & (1 << **id) != 0)
        .map(|(_id, valve)| valve.flow * 26)
        .sum();
    let initial = State { location: aa_id, minutes: 26, closed: closed_valves };
    let (_path, elephant_cost) = dijkstra(&initial, successors, success).unwrap();
    let elephant_flow = elephant_max_flow - elephant_cost;

    person_flow + elephant_flow
}

fn all_pairs_shortest_paths(input: &HashMap<ValveID, Valve>) -> HashMap<(ValveID, ValveID), u32> {
    let mut result = HashMap::new();

    // We're going to do this the expensive way: via dijkstra_all
    for source in input.keys() {
        let successors = |node: &ValveID| -> Vec<(ValveID, u32)> {
            input[node].neighbors.iter().map(|name| (*name, 1)).collect()
        };
        let paths = dijkstra_all(source, successors);
        for (destination, (_, cost)) in paths.iter() {
            result.insert((*source, *destination), *cost);
        }
    }

    result
}

type ValveID = u32;
#[derive(Debug)]
struct Valve {
    flow: u32,
    neighbors: Vec<ValveID>
}

fn parse_input(input: &str) -> (ValveID, HashMap<ValveID, Valve>) {
    // Build a mapping from textual valve name to ValveID,
    // All valve names are two characters, so I can slice the input
    // to get the name.
    let mut valve_names = HashMap::<&str, ValveID>::new();
    for (id,line) in input.lines().enumerate() {
        valve_names.insert(&line[6..8], id as ValveID);
    }

    let mut result = HashMap::new();

    for line in input.lines() {
        // Get this valve's ID.
        let id = valve_names[&line[6..8]];

        // Get this valve's flow rate.  The flow rate starts at a fixed
        // column.  Find the semicolon and parse that range.
        let semicolon = line.find(';').unwrap();
        let flow = line[23..semicolon].parse().unwrap();

        // Get the IDs of neighbor valves.  Darned input has both
        // "lead to valve " and "lead to valves ".
        let mut neighbor_offset = line.find("to valve").unwrap() + 8;
        if line[neighbor_offset..].starts_with('s') {
            neighbor_offset += 2;   // Skip over "s "
        } else {
            neighbor_offset += 1;   // Skip over " "
        }
        let neighbors = line[neighbor_offset..]
            .split(", ")
            .map(|s| valve_names[s])
            .collect();
        let valve = Valve{flow, neighbors};
        // println!("{name}: {valve:?}");
        result.insert(id, valve);
    }

    (valve_names["AA"], result)
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
