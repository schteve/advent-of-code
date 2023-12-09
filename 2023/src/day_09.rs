/*

*/

fn extrapolate(values: &[i32], forward: bool) -> i32 {
    let mut serieses: Vec<Vec<i32>> = vec![values.to_vec()];

    loop {
        let new_series: Vec<i32> = serieses
            .last()
            .unwrap()
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect();
        let all_zero = new_series.iter().all(|v| *v == 0);
        if all_zero {
            break;
        }
        serieses.push(new_series);
    }

    let mut extrapolated = 0;
    while let Some(series) = serieses.pop() {
        if forward {
            extrapolated += series.last().unwrap();
        } else {
            extrapolated = series.first().unwrap() - extrapolated;
        }
    }
    extrapolated
}

fn extrapolate_all(serieses: &[Vec<i32>], forward: bool) -> i32 {
    serieses
        .iter()
        .map(|series| extrapolate(series, forward))
        .sum()
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|tok| tok.parse().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Vec<i32>]) -> i32 {
    let value = extrapolate_all(input, true);
    assert_eq!(value, 1980437560);
    value
}

#[aoc(day9, part2)]
pub fn part2(input: &[Vec<i32>]) -> i32 {
    let value = extrapolate_all(input, false);
    assert_eq!(value, 977);
    value
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_extrapolate_all() {
        let input = input_generator(EXAMPLE_INPUT);
        let value = extrapolate_all(&input, true);
        assert_eq!(value, 114);
        let value = extrapolate_all(&input, false);
        assert_eq!(value, 2);
    }
}
