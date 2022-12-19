/*
    --- Day 16: Proboscidea Volcanium ---
    The sensors have led you to the origin of the distress signal: yet another handheld device, just like the one the Elves gave you. However, you don't see any Elves around; instead, the device is surrounded by elephants! They must have gotten lost in these tunnels, and one of the elephants apparently figured out how to turn on the distress signal.

    The ground rumbles again, much stronger this time. What kind of cave is this, exactly? You scan the cave with your handheld device; it reports mostly igneous rock, some ash, pockets of pressurized gas, magma... this isn't just a cave, it's a volcano!

    You need to get the elephants out of here, quickly. Your device estimates that you have 30 minutes before the volcano erupts, so you don't have time to go back out the way you came in.

    You scan the cave for other options and discover a network of pipes and pressure-release valves. You aren't sure how such a system got into a volcano, but you don't have time to complain; your device produces a report (your puzzle input) of each valve's flow rate if it were opened (in pressure per minute) and the tunnels you could use to move between the valves.

    There's even a valve in the room you and the elephants are currently standing in labeled AA. You estimate it will take you one minute to open a single valve and one minute to follow any tunnel from one valve to another. What is the most pressure you could release?

    For example, suppose you had the following scan output:

    Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II
    All of the valves begin closed. You start at valve AA, but it must be damaged or jammed or something: its flow rate is 0, so there's no point in opening it. However, you could spend one minute moving to valve BB and another minute opening it; doing so would release pressure during the remaining 28 minutes at a flow rate of 13, a total eventual pressure release of 28 * 13 = 364. Then, you could spend your third minute moving to valve CC and your fourth minute opening it, providing an additional 26 minutes of eventual pressure release at a flow rate of 2, or 52 total pressure released by valve CC.

    Making your way through the tunnels like this, you could probably open many or all of the valves by the time 30 minutes have elapsed. However, you need to release as much pressure as possible, so you'll need to be methodical. Instead, consider this approach:

    == Minute 1 ==
    No valves are open.
    You move to valve DD.

    == Minute 2 ==
    No valves are open.
    You open valve DD.

    == Minute 3 ==
    Valve DD is open, releasing 20 pressure.
    You move to valve CC.

    == Minute 4 ==
    Valve DD is open, releasing 20 pressure.
    You move to valve BB.

    == Minute 5 ==
    Valve DD is open, releasing 20 pressure.
    You open valve BB.

    == Minute 6 ==
    Valves BB and DD are open, releasing 33 pressure.
    You move to valve AA.

    == Minute 7 ==
    Valves BB and DD are open, releasing 33 pressure.
    You move to valve II.

    == Minute 8 ==
    Valves BB and DD are open, releasing 33 pressure.
    You move to valve JJ.

    == Minute 9 ==
    Valves BB and DD are open, releasing 33 pressure.
    You open valve JJ.

    == Minute 10 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve II.

    == Minute 11 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve AA.

    == Minute 12 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve DD.

    == Minute 13 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve EE.

    == Minute 14 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve FF.

    == Minute 15 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve GG.

    == Minute 16 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You move to valve HH.

    == Minute 17 ==
    Valves BB, DD, and JJ are open, releasing 54 pressure.
    You open valve HH.

    == Minute 18 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You move to valve GG.

    == Minute 19 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You move to valve FF.

    == Minute 20 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You move to valve EE.

    == Minute 21 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You open valve EE.

    == Minute 22 ==
    Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
    You move to valve DD.

    == Minute 23 ==
    Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
    You move to valve CC.

    == Minute 24 ==
    Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
    You open valve CC.

    == Minute 25 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    == Minute 26 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    == Minute 27 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    == Minute 28 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    == Minute 29 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    == Minute 30 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
    This approach lets you release the most pressure possible in 30 minutes with this valve layout, 1651.

    Work out the steps to release the most pressure in 30 minutes. What is the most pressure you can release?

    --- Part Two ---
    You're worried that even with an optimal approach, the pressure released won't be enough. What if you got one of the elephants to help you?

    It would take you 4 minutes to teach an elephant how to open the right valves in the right order, leaving you with only 26 minutes to actually execute your plan. Would having two of you working together be better, even if it means having less time? (Assume that you teach the elephant before opening any valves yourself, giving you both the same full 26 minutes.)

    In the example above, you could teach the elephant to help you as follows:

    == Minute 1 ==
    No valves are open.
    You move to valve II.
    The elephant moves to valve DD.

    == Minute 2 ==
    No valves are open.
    You move to valve JJ.
    The elephant opens valve DD.

    == Minute 3 ==
    Valve DD is open, releasing 20 pressure.
    You open valve JJ.
    The elephant moves to valve EE.

    == Minute 4 ==
    Valves DD and JJ are open, releasing 41 pressure.
    You move to valve II.
    The elephant moves to valve FF.

    == Minute 5 ==
    Valves DD and JJ are open, releasing 41 pressure.
    You move to valve AA.
    The elephant moves to valve GG.

    == Minute 6 ==
    Valves DD and JJ are open, releasing 41 pressure.
    You move to valve BB.
    The elephant moves to valve HH.

    == Minute 7 ==
    Valves DD and JJ are open, releasing 41 pressure.
    You open valve BB.
    The elephant opens valve HH.

    == Minute 8 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You move to valve CC.
    The elephant moves to valve GG.

    == Minute 9 ==
    Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
    You open valve CC.
    The elephant moves to valve FF.

    == Minute 10 ==
    Valves BB, CC, DD, HH, and JJ are open, releasing 78 pressure.
    The elephant moves to valve EE.

    == Minute 11 ==
    Valves BB, CC, DD, HH, and JJ are open, releasing 78 pressure.
    The elephant opens valve EE.

    (At this point, all valves are open.)

    == Minute 12 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    ...

    == Minute 20 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.

    ...

    == Minute 26 ==
    Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
    With the elephant helping, after 26 minutes, the best you could do would release a total of 1707 pressure.

    With you and an elephant working together for 26 minutes, what is the most pressure you could release?
*/

use std::cmp::Reverse;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Valve {
    name: String,
    flow_rate: u64,
    tunnels: Vec<String>,
}

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ((?:\w+(?:, )?)+)"
    )
    .unwrap();
}

impl Valve {
    fn from_str(s: &str) -> Self {
        let caps = RE.captures(s).unwrap();
        let name = caps.get(1).unwrap().as_str().to_owned();
        let flow_rate = caps.get(2).unwrap().as_str().parse().unwrap();
        let tunnels_str = caps.get(3).unwrap().as_str();
        let tunnels = tunnels_str.split(", ").map(ToOwned::to_owned).collect();

        Self {
            name,
            flow_rate,
            tunnels,
        }
    }
}

// TODO: try the DP in reverse to reduce the work. Instead of setting future cells,
// refer exclusively to past ones (path A, path B, and do-nothing).
// TODO: make DP not use option as this may be a big CPU cycle burner

#[derive(Debug)]
struct ValveNode<'a> {
    name: &'a str,
    flow_rate: u64,
    tunnels: Vec<usize>,
}

impl<'a> ValveNode<'a> {
    fn new(name: &'a str, flow_rate: u64) -> Self {
        Self {
            name,
            flow_rate,
            tunnels: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Network<'a> {
    nodes: Vec<ValveNode<'a>>,
    shortest_paths: Vec<Vec<Option<u32>>>,
    n_valves_with_flow: usize,
}

impl<'a> Network<'a> {
    fn from_valves(valves: &'a [Valve]) -> Self {
        // First allocate all nodes so we can reference their IDs.
        // Sorting these in reverse order puts all valves with flow > 0 in the beginning which means we can use
        // non-sparse bitmasks when we want to only deal with them.
        let mut nodes: Vec<ValveNode<'a>> = valves
            .iter()
            .map(|valve| ValveNode::new(&valve.name, valve.flow_rate))
            .sorted_by_key(|valve_node| Reverse(valve_node.flow_rate))
            .collect();
        let get_idx = |nodes: &[ValveNode<'a>], name: &str| {
            nodes.iter().position(|n| n.name == name).unwrap()
        };

        // Add children (tunnels) to each node according to their ID (index)
        for valve in valves {
            let valve_idx = get_idx(&nodes, &valve.name);
            nodes[valve_idx].tunnels = valve
                .tunnels
                .iter()
                .map(|tunnel| get_idx(&nodes, tunnel))
                .collect();
        }

        // Calculate adjacency matrix from the graph as this is the starting point for Floyd-Warshall
        let mut adjacency_matrix: Vec<Vec<Option<u32>>> =
            vec![vec![None; nodes.len()]; nodes.len()];
        for (i, node) in nodes.iter().enumerate() {
            adjacency_matrix[i][i] = Some(0); // Not clear if this should be 0, None, or if it even matters
            for tunnel in &node.tunnels {
                adjacency_matrix[i][*tunnel] = Some(1); // All distances are 1 by definition
            }
        }

        // Floyd-Warshall to get shortest distance between each node pair
        let mut shortest_paths = adjacency_matrix; // TODO: collect the option out, after fw all items are Some in a fully connected graph
        for k in 0..nodes.len() {
            for i in 0..nodes.len() {
                for j in 0..nodes.len() {
                    shortest_paths[i][j] = match (
                        shortest_paths[i][j],
                        shortest_paths[i][k],
                        shortest_paths[k][j],
                    ) {
                        (None, Some(ik), Some(kj)) => Some(ik + kj), // No existing path, use new one
                        (Some(curr), Some(ik), Some(kj)) => Some(curr.min(ik + kj)), // Shorter of existing path and new one
                        (x, _, _) => x, // No new path, use existing (if any)
                    };
                }
            }
        }

        // Since the nodes are sorted by flow rate, the first zero is the count of non-zero flows
        let n_valves_with_flow = nodes.iter().position(|node| node.flow_rate == 0).unwrap();

        Self {
            nodes,
            shortest_paths,
            n_valves_with_flow,
        }
    }

    fn find_max_pressures(&self, max_time: usize) -> Vec<Vec<Vec<Option<u64>>>> {
        // Time left, current node, state
        let mut dp: Vec<Vec<Vec<Option<u64>>>> =
            vec![vec![vec![None; 1 << self.n_valves_with_flow]; self.n_valves_with_flow]; max_time];
        // TODO: make this a flat vec

        // Seed first moves from AA since AA isn't in the DP matrix (due to no flow)
        let aa_id = self
            .nodes
            .iter()
            .position(|n| n.name == "AA")
            .expect("No AA??");
        for valve_id in 0..self.n_valves_with_flow {
            let next_time = self.shortest_paths[aa_id][valve_id].unwrap() as usize + 1; // Time to travel to the valve + 1 to turn it on
            let next_value = (max_time - next_time) as u64 * self.nodes[valve_id].flow_rate;
            /*println!(
                "Seed DP[{next_time}][{valve_id}][{}] = {next_value}",
                1 << valve_id
            );*/
            dp[next_time][valve_id][1 << valve_id] = Some(next_value);
        }

        let set_if_higher = |curr_val: &mut Option<u64>, new_val: Option<u64>| {
            if let Some(curr) = curr_val {
                if let Some(new) = new_val {
                    *curr = (*curr).max(new);
                }
            } else {
                *curr_val = new_val;
            }
        };

        // DP
        for curr_time in 0..max_time {
            for node_id in 0..self.n_valves_with_flow {
                for bitmask in 0..(1 << self.n_valves_with_flow) {
                    /*println!(
                        "At DP[{curr_time}][{node_id}][{bitmask}] = {:?}",
                        dp[curr_time][node_id][bitmask]
                    );*/

                    // For each closed valve, travel to it and open it
                    for valve_id in 0..self.n_valves_with_flow {
                        let bit = 1 << valve_id;
                        if bitmask & bit == 0 {
                            let next_time = curr_time
                                + self.shortest_paths[node_id][valve_id].unwrap() as usize
                                + 1; // Time to travel to the valve + 1 to turn it on
                            if next_time < max_time {
                                let next_value = dp[curr_time][node_id][bitmask].map(|v| {
                                    v + (max_time - next_time) as u64
                                        * self.nodes[valve_id].flow_rate
                                });
                                /*println!(
                                    "Open DP[{next_time}][{valve_id}][{}] = {next_value:?}",
                                    bitmask | bit
                                );*/
                                set_if_higher(
                                    &mut dp[next_time][valve_id][bitmask | bit],
                                    next_value,
                                );
                            }
                        }
                    }

                    // Don't do anything (not clear if this is actually needed)
                    let next_time = curr_time + 1;
                    if next_time < max_time {
                        let next_value = dp[curr_time][node_id][bitmask];
                        //println!("Wait DP[{next_time}][{node_id}][{bitmask}] = {next_value:?}");
                        set_if_higher(&mut dp[next_time][node_id][bitmask], next_value);
                    }
                }
            }
        }

        //print_dp(&self.nodes, &dp, n_valves_with_flow);

        /*let print_step = |time: usize, node_name: &str, valves_on: &[&str]| {
            let get_idx = |name: &str| self.nodes.iter().position(|n| n.name == name).unwrap();

            let node_id = get_idx(node_name);
            let mut bitmask = 0;
            for valve in valves_on {
                let valve_id = get_idx(valve);
                bitmask |= 1 << valve_id;
            }
            println!(
                "DP[{time}][{node_name}][{bitmask}] = {:?}",
                dp[time][node_id][bitmask]
            );
        };

        print_step(2, "DD", &["DD"]);
        print_step(5, "BB", &["DD", "BB"]);
        print_step(9, "JJ", &["DD", "BB", "JJ"]);
        print_step(17, "HH", &["DD", "BB", "JJ", "HH"]);
        print_step(21, "EE", &["DD", "BB", "JJ", "HH", "EE"]);
        print_step(24, "CC", &["DD", "BB", "JJ", "HH", "EE", "CC"]);*/

        dp
    }

    fn release_pressure(&self, max_time: usize) -> u64 {
        let dp = self.find_max_pressures(max_time);
        dp[max_time - 1]
            .iter()
            .flat_map(|x| x.iter())
            .max()
            .unwrap()
            .unwrap()
    }

    fn release_pressure_with_help(&self, max_time: usize) -> u64 {
        let dp = self.find_max_pressures(max_time);
        let mut max = 0;
        for me_bitmask in 0..(1 << self.n_valves_with_flow) {
            for elephant_bitmask in 0..(1 << self.n_valves_with_flow) {
                if me_bitmask & elephant_bitmask == 0 {
                    for me_node in 0..self.n_valves_with_flow {
                        if me_bitmask & (1 << me_node) != 0 {
                            for elephant_node in 0..self.n_valves_with_flow {
                                if elephant_bitmask & (1 << elephant_node) != 0 {
                                    if let (Some(a), Some(b)) = (
                                        dp[max_time - 1][me_node][me_bitmask],
                                        dp[max_time - 1][elephant_node][elephant_bitmask],
                                    ) {
                                        max = max.max(a + b);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        max
    }
}

/*fn print_dp(nodes: &[ValveNode], dp: &Vec<Vec<Vec<Option<u64>>>>, n_valves_with_flow: usize) {
    println!();
    for time_used in 0..5 {
        println!("TIME = {time_used}");
        for node_id in 0..n_valves_with_flow {
            print!("    Node {}: ", nodes[node_id].name);
            for bitmask in 0..(1 << n_valves_with_flow) {
                if let Some(val) = dp[time_used][node_id][bitmask] {
                    print!("{:3} ", val);
                } else {
                    print!("  N ");
                }
                if bitmask == (1 << (n_valves_with_flow - 1)) - 1 {
                    println!();
                    print!("             ");
                }
            }
            println!();
        }
    }
}*/

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Vec<Valve> {
    input.lines().map(Valve::from_str).collect()
}

#[aoc(day16, part1)]
pub fn part1(input: &[Valve]) -> u64 {
    let network = Network::from_valves(input);
    let max = network.release_pressure(30);
    assert_eq!(max, 1947);
    max
}

#[aoc(day16, part2)]
pub fn part2(input: &[Valve]) -> u64 {
    let network = Network::from_valves(input);
    let max = network.release_pressure_with_help(26);
    assert_eq!(max, 2556);
    max
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
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
    fn test_release_pressure() {
        let input = input_generator(EXAMPLE_INPUT);
        let network = Network::from_valves(&input);
        let max = network.release_pressure(30);
        assert_eq!(max, 1651);
    }

    #[test]
    fn test_release_pressure_with_help() {
        let input = input_generator(EXAMPLE_INPUT);
        let network = Network::from_valves(&input);
        let max = network.release_pressure_with_help(26);
        assert_eq!(max, 1707);
    }
}
