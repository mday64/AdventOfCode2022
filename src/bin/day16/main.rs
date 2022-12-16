use std::collections::HashMap;
use pathfinding::prelude::dijkstra_all;

fn main() {
    let path = std::env::args().skip(1).next()
        .unwrap_or("src/bin/day16/input.txt".into());
    let input = std::fs::read_to_string(path).unwrap();

    let result1 = part1(&input);
    println!("Part 1: {}", result1);
    assert_eq!(result1, 1701);
}

//
// Part 1
//
// Let's use Dijkstra's shortest path algorithm.  Let's hope that
// pathfinding's algorithm works with negative numbers since we want
// a maximum and the algorithm finds a minimum.  Therefore, we'll use
// negative costs.
//
// The state will be:
//  location (valve name)
//  time remaining (minutes)
//  valves already open (HashSet<String>)
//
// Cost is -flow * minutes remaining (for opening a valve); zero for movement
//
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    location: String,
    minutes: i32,
    opened: Vec<String>
}
fn part1(input: &str) -> i32 {
    let valves = parse_input(input);
    let initial = State { location: "AA".to_string(), minutes: 30, opened: vec![] };
    // let success = |state: &State| state.minutes == 0;
    let successors = |state: &State| -> Vec<(State, i32)> {
        if state.minutes == 0 {
            return vec![];
        }

        // dbg!(&state.opened);

        let mut next_states = vec![];

        // If the current valve is not yet open and its flow rate is non-zero,
        // then one possibility is to spend one minute opening the valve.
        // Remember that cost is negative.  We subtract 1 because the flow
        // doesn't begin until the start of the next minute.
        if valves[&state.location].flow > 0 && !state.opened.contains(&state.location) {
            let mut opened = state.opened.clone();
            opened.push(state.location.clone());
            next_states.push((
                State{
                    location: state.location.clone(),
                    minutes: state.minutes - 1,
                    opened
                },
                -(state.minutes-1)*valves[&state.location].flow
            ));
        }

        // We can move to a location with a different valve
        for neighbor in valves[&state.location].neighbors.iter() {
            next_states.push((
                State {
                    location: neighbor.clone(),
                    minutes: state.minutes - 1,
                    opened: state.opened.clone()
                }, 0
            ));
        }

        next_states
    };
    
    // Because "success" is defined as running out of time, we need to call
    // dijkstra_all to examine all possibilities.  Then find the best flow
    // from all of those.
    let result = dijkstra_all(&initial, successors);
    result.values().map(|(_state, cost)| -cost).max().unwrap()
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
