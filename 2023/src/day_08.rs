/*
    --- Day 8: Haunted Wasteland ---
    You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

    One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

    It seems like you're meant to use the left/right instructions to navigate the network. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

    After examining the maps for a bit, two nodes stick out: AAA and ZZZ. You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

    This format defines each node of the network individually. For example:

    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    Starting with AAA, you need to look up the next element based on the next left/right instruction in your input. In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

    Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?

    --- Part Two ---
    The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!

    What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.

    After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z! If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.

    For example:

    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction, use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

    Step 0: You are at 11A and 22A.
    Step 1: You choose all of the left paths, leading you to 11B and 22B.
    Step 2: You choose all of the right paths, leading you to 11Z and 22C.
    Step 3: You choose all of the left paths, leading you to 11B and 22Z.
    Step 4: You choose all of the right paths, leading you to 11Z and 22B.
    Step 5: You choose all of the left paths, leading you to 11B and 22C.
    Step 6: You choose all of the right paths, leading you to 11Z and 22Z.
    So, in this example, you end up entirely on nodes that end in Z after 6 steps.

    Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?
*/

use std::collections::HashMap;
#[cfg(test)]
use std::collections::HashSet;

#[cfg(test)]
use hashbag::HashBag;

use num::integer::lcm;

use common::Turn;

struct Node {
    left: String,
    right: String,
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let no_parens = value.trim_start_matches('(').trim_end_matches(')');
        let (left, right) = no_parens.split_once(',').unwrap();
        Self {
            left: left.trim().to_owned(),
            right: right.trim().to_owned(),
        }
    }
}

pub struct Network {
    instructions: Vec<Turn>,
    nodes: HashMap<String, Node>,
}

impl Network {
    fn traverse(&self) -> usize {
        let start = "AAA";
        let end = "ZZZ";

        let mut curr = start;
        for (i, turn) in self.instructions.iter().cycle().enumerate() {
            if curr == end {
                return i;
            }

            let curr_node = self.nodes.get(curr).unwrap();
            curr = match turn {
                Turn::Left => &curr_node.left,
                Turn::Right => &curr_node.right,
            }
        }
        unreachable!("Loops forever");
    }

    #[cfg(test)]
    fn inspect_ghostly(&self) {
        let currs: Vec<&str> = self
            .nodes
            .keys()
            .filter_map(|name| {
                if name.ends_with('A') {
                    Some(name.as_str())
                } else {
                    None
                }
            })
            .collect();
        for curr in currs {
            let mut curr = curr;
            let mut seen: HashSet<(usize, &str)> = HashSet::new();
            let mut count_z = HashBag::new();
            let mut zs = Vec::new();
            for (i, (turn_i, turn)) in self.instructions.iter().enumerate().cycle().enumerate() {
                //println!("{i}: {curr} {turn_i} {turn}");
                if curr.ends_with('Z') {
                    count_z.insert(curr);
                    zs.push(i);
                    if zs.len() == 2 {
                        println!("Z's at {zs:?}");
                        println!("Z's are {} apart", zs[1] - zs[0]);
                        assert_eq!(zs[1] - zs[0], zs[0]);
                        break;
                    }
                }

                if !seen.insert((turn_i, curr)) {
                    println!("Cycle after {i} at {curr}");

                    assert_eq!(count_z.set_len(), 1);
                    let (s, count) = count_z.set_iter().next().unwrap();
                    println!("Saw {} {}'s", count, s);

                    seen.clear(); // Keep going till we find two Z's
                }

                let curr_node = self.nodes.get(curr).unwrap();
                curr = match turn {
                    Turn::Left => curr_node.left.as_str(),
                    Turn::Right => curr_node.right.as_str(),
                };
            }
        }
    }

    fn traverse_ghostly(&self) -> usize {
        // We assume, based on inspecting the data, that there is only one type
        // of Z findable from each A (but may appear multiple times per cycle),
        // and that  Z's appear at regular intervals from both the A's and from
        // each other.

        let currs: Vec<&str> = self
            .nodes
            .keys()
            .filter_map(|name| {
                if name.ends_with('A') {
                    Some(name.as_str())
                } else {
                    None
                }
            })
            .collect();

        let cycles: Vec<usize> = currs
            .into_iter()
            .map(|mut curr| {
                let mut retval = 0;
                for (i, turn) in self.instructions.iter().cycle().enumerate() {
                    if curr.ends_with('Z') {
                        retval = i;
                        break;
                    }

                    let curr_node = self.nodes.get(curr).unwrap();
                    curr = match turn {
                        Turn::Left => &curr_node.left,
                        Turn::Right => &curr_node.right,
                    }
                }
                retval
            })
            .collect();

        cycles.iter().fold(1, |acc, x| lcm(acc, *x))
    }
}

impl From<&str> for Network {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();

        let instructions = lines.next().unwrap().chars().map(|c| c.into()).collect();
        let _ = lines.next().unwrap();

        let nodes = lines
            .map(|line| {
                let (left, right) = line.split_once('=').unwrap();
                let name = left.trim().to_string();
                let node: Node = right.trim().into();
                (name, node)
            })
            .collect();

        Self {
            instructions,
            nodes,
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Network {
    input.into()
}

#[aoc(day8, part1)]
pub fn part1(input: &Network) -> usize {
    let value = input.traverse();
    assert_eq!(value, 13301);
    value
}

#[aoc(day8, part2)]
pub fn part2(input: &Network) -> usize {
    //input.inspect_ghostly();
    let value = input.traverse_ghostly();
    assert_eq!(value, 7309459565207);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT_1: &str = "\
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    static EXAMPLE_INPUT_2: &str = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    static EXAMPLE_INPUT_3: &str = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_traverse() {
        let input = input_generator(EXAMPLE_INPUT_1);
        let value = input.traverse();
        assert_eq!(value, 2);

        let input = input_generator(EXAMPLE_INPUT_2);
        let value = input.traverse();
        assert_eq!(value, 6);
    }

    #[test]
    fn test_inspect_ghostly() {
        let input = input_generator(EXAMPLE_INPUT_3);
        input.inspect_ghostly();
    }

    #[test]
    fn test_traverse_ghostly() {
        let input = input_generator(EXAMPLE_INPUT_3);
        let value = input.traverse_ghostly();
        assert_eq!(value, 6);
    }
}
