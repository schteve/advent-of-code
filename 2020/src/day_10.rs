/*
    --- Day 10: Adapter Array ---
    Patched into the aircraft's data port, you discover weather forecasts of a massive tropical storm. Before you can figure out whether it will impact your vacation plans, however, your device suddenly turns off!

    Its battery is dead.

    You'll need to plug it in. There's only one problem: the charging outlet near your seat produces the wrong number of jolts. Always prepared, you make a list of all of the joltage adapters in your bag.

    Each of your joltage adapters is rated for a specific output joltage (your puzzle input). Any given adapter can take an input 1, 2, or 3 jolts lower than its rating and still produce its rated output joltage.

    In addition, your device has a built-in joltage adapter rated for 3 jolts higher than the highest-rated adapter in your bag. (If your adapter list were 3, 9, and 6, your device's built-in adapter would be rated for 12 jolts.)

    Treat the charging outlet near your seat as having an effective joltage rating of 0.

    Since you have some time to kill, you might as well test all of your adapters. Wouldn't want to get to your resort and realize you can't even charge your device!

    If you use every adapter in your bag at once, what is the distribution of joltage differences between the charging outlet, the adapters, and your device?

    For example, suppose that in your bag, you have adapters with the following joltage ratings:

    16
    10
    15
    5
    1
    11
    7
    19
    6
    12
    4
    With these adapters, your device's built-in joltage adapter would be rated for 19 + 3 = 22 jolts, 3 higher than the highest-rated adapter.

    Because adapters can only connect to a source 1-3 jolts lower than its rating, in order to use every adapter, you'd need to choose them like this:

    The charging outlet has an effective rating of 0 jolts, so the only adapters that could connect to it directly would need to have a joltage rating of 1, 2, or 3 jolts. Of these, only one you have is an adapter rated 1 jolt (difference of 1).
    From your 1-jolt rated adapter, the only choice is your 4-jolt rated adapter (difference of 3).
    From the 4-jolt rated adapter, the adapters rated 5, 6, or 7 are valid choices. However, in order to not skip any adapters, you have to pick the adapter rated 5 jolts (difference of 1).
    Similarly, the next choices would need to be the adapter rated 6 and then the adapter rated 7 (with difference of 1 and 1).
    The only adapter that works with the 7-jolt rated adapter is the one rated 10 jolts (difference of 3).
    From 10, the choices are 11 or 12; choose 11 (difference of 1) and then 12 (difference of 1).
    After 12, only valid adapter has a rating of 15 (difference of 3), then 16 (difference of 1), then 19 (difference of 3).
    Finally, your device's built-in adapter is always 3 higher than the highest adapter, so its rating is 22 jolts (always a difference of 3).
    In this example, when using every adapter, there are 7 differences of 1 jolt and 5 differences of 3 jolts.

    Here is a larger example:

    28
    33
    18
    42
    31
    14
    46
    20
    48
    47
    24
    23
    49
    45
    19
    38
    39
    11
    1
    32
    25
    35
    8
    17
    7
    9
    4
    2
    34
    10
    3
    In this larger example, in a chain that uses all of the adapters, there are 22 differences of 1 jolt and 10 differences of 3 jolts.

    Find a chain that uses all of your adapters to connect the charging outlet to your device's built-in adapter and count the joltage differences between the charging outlet, the adapters, and your device. What is the number of 1-jolt differences multiplied by the number of 3-jolt differences?

    --- Part Two ---
    To completely determine whether you have enough adapters, you'll need to figure out how many different ways they can be arranged. Every arrangement needs to connect the charging outlet to your device. The previous rules about when adapters can successfully connect still apply.

    The first example above (the one that starts with 16, 10, 15) supports the following arrangements:

    (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
    (0), 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, (22)
    (0), 1, 4, 5, 7, 10, 11, 12, 15, 16, 19, (22)
    (0), 1, 4, 5, 7, 10, 12, 15, 16, 19, (22)
    (0), 1, 4, 6, 7, 10, 11, 12, 15, 16, 19, (22)
    (0), 1, 4, 6, 7, 10, 12, 15, 16, 19, (22)
    (0), 1, 4, 7, 10, 11, 12, 15, 16, 19, (22)
    (0), 1, 4, 7, 10, 12, 15, 16, 19, (22)
    (The charging outlet and your device's built-in adapter are shown in parentheses.) Given the adapters from the first example, the total number of arrangements that connect the charging outlet to your device is 8.

    The second example above (the one that starts with 28, 33, 18) has many arrangements. Here are a few:

    (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
    32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)

    (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
    32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 49, (52)

    (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
    32, 33, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)

    (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
    32, 33, 34, 35, 38, 39, 42, 45, 46, 49, (52)

    (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
    32, 33, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)

    (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
    46, 48, 49, (52)

    (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
    46, 49, (52)

    (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
    47, 48, 49, (52)

    (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
    47, 49, (52)

    (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
    48, 49, (52)
    In total, this set of adapters can connect the charging outlet to your device in 19208 distinct arrangements.

    You glance back down at your bag and try to remember why you brought so many adapters; there must be more than a trillion valid ways to arrange them! Surely, there must be an efficient way to count the arrangements.

    What is the total number of distinct ways you can arrange the adapters to connect the charging outlet to your device?
*/

fn find_jolt_differences(values: &[u32]) -> (usize, usize, usize) {
    let mut sorted_values = values.to_vec();
    sorted_values.push(0); // Charging outlet starts at 0
    sorted_values.sort_unstable();
    sorted_values.push(sorted_values.last().unwrap() + 3); // The built-in adapter is always 3 higher

    // dbg!(&sorted_values);
    sorted_values.windows(2).fold((0, 0, 0), |acc, window| {
        let diff = window[1] - window[0];
        if diff == 1 {
            (acc.0 + 1, acc.1, acc.2)
        } else if diff == 2 {
            (acc.0, acc.1 + 1, acc.2)
        } else if diff == 3 {
            (acc.0, acc.1, acc.2 + 1)
        } else {
            panic!("Jolt difference too high: {}", diff);
        }
    })
}

fn count_arrangements(values: &[u32]) -> usize {
    let mut sorted_values = values.to_vec();
    sorted_values.push(0); // Charging outlet starts at 0
    sorted_values.sort_unstable();

    // Use a basic version of Dijkstra's to calculate the number of paths, where only the count of each path
    // currently ending at each node is maintained (we don't care about the path itself, just how many there are).
    let mut paths: Vec<usize> = vec![0; sorted_values.len()];
    paths[0] = 1;

    // Process each node in order. This acts identically to a priority queue since the nodes only connect forward.
    // Skip the last index since there's nothing following it, so no point checking it.
    for i in 0..sorted_values.len() - 1 {
        if i + 1 < sorted_values.len() {
            // Next node is always an option
            paths[i + 1] += paths[i];
        }
        if i + 2 < sorted_values.len() && sorted_values[i + 2] - sorted_values[i] <= 3 {
            paths[i + 2] += paths[i];
        }
        if i + 3 < sorted_values.len() && sorted_values[i + 3] - sorted_values[i] <= 3 {
            paths[i + 3] += paths[i];
        }
        paths[i] = 0;
    }

    // The last index always contains the complete count
    *paths.last().unwrap()
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect()
}

#[aoc(day10, part1)]
pub fn part1(input: &[u32]) -> usize {
    let (one, _two, three) = find_jolt_differences(input);
    let product = one * three;
    println!("1-jolt x 3-jolt: {}", product);
    assert_eq!(product, 2030);
    product
}

#[aoc(day10, part2)]
pub fn part2(input: &[u32]) -> usize {
    let arrangements = count_arrangements(input);
    println!("Arrangements: {}", arrangements);
    assert_eq!(arrangements, 42313823813632);
    arrangements
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT1: &str = "\
16
10
15
5
1
11
7
19
6
12
4";

    static EXAMPLE_INPUT2: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_find_jolt_differences() {
        let input = input_generator(EXAMPLE_INPUT1);
        let (one, _two, three) = find_jolt_differences(&input);
        assert_eq!(one, 7);
        assert_eq!(three, 5);

        let input = input_generator(EXAMPLE_INPUT2);
        let (one, _two, three) = find_jolt_differences(&input);
        assert_eq!(one, 22);
        assert_eq!(three, 10);
    }

    #[test]
    fn test_count_arrangements() {
        let input = input_generator(EXAMPLE_INPUT1);
        let arrangements = count_arrangements(&input);
        assert_eq!(arrangements, 8);

        let input = input_generator(EXAMPLE_INPUT2);
        let arrangements = count_arrangements(&input);
        assert_eq!(arrangements, 19208);
    }
}
