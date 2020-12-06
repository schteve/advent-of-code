/*
    --- Day 5: Binary Boarding ---
    You board your plane only to discover a new problem: you dropped your boarding pass! You aren't sure which seat is yours, and all of the flight attendants are busy with the flood of people that suddenly made it through passport control.

    You write a quick program to use your phone's camera to scan all of the nearby boarding passes (your puzzle input); perhaps you can find your seat through process of elimination.

    Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R means "right".

    The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in. Start with the whole list of rows; the first letter indicates whether the seat is in the front (0 through 63) or the back (64 through 127). The next letter indicates which half of that region the seat is in, and so on until you're left with exactly one row.

    For example, consider just the first seven characters of FBFBBFFRLR:

    Start by considering the whole range, rows 0 through 127.
    F means to take the lower half, keeping rows 0 through 63.
    B means to take the upper half, keeping rows 32 through 63.
    F means to take the lower half, keeping rows 32 through 47.
    B means to take the upper half, keeping rows 40 through 47.
    B keeps rows 44 through 47.
    F keeps rows 44 through 45.
    The final F keeps the lower of the two, row 44.
    The last three characters will be either L or R; these specify exactly one of the 8 columns of seats on the plane (numbered 0 through 7). The same process as above proceeds again, this time with only three steps. L means to keep the lower half, while R means to keep the upper half.

    For example, consider just the last 3 characters of FBFBBFFRLR:

    Start by considering the whole range, columns 0 through 7.
    R means to take the upper half, keeping columns 4 through 7.
    L means to take the lower half, keeping columns 4 through 5.
    The final R keeps the upper of the two, column 5.
    So, decoding FBFBBFFRLR reveals that it is the seat at row 44, column 5.

    Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID 44 * 8 + 5 = 357.

    Here are some other boarding passes:

    BFFFBBFRRR: row 70, column 7, seat ID 567.
    FFFBBBFRRR: row 14, column 7, seat ID 119.
    BBFFBBFRLL: row 102, column 4, seat ID 820.
    As a sanity check, look through your list of boarding passes. What is the highest seat ID on a boarding pass?

    --- Part Two ---
    Ding! The "fasten seat belt" signs have turned on. Time to find your seat.

    It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.

    Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.

    What is the ID of your seat?
*/

pub struct BoardingPass {
    seat_bsp: String,
}

impl BoardingPass {
    fn from_string(input: &str) -> Self {
        Self {
            seat_bsp: input[..10].to_owned(),
        }
    }

    fn row(&self) -> u32 {
        let mut value = 0u32;
        for c in self.seat_bsp[..7].chars() {
            value <<= 1;
            match c {
                'F' => value |= 0,
                'B' => value |= 1,
                _ => panic!("Unknown row character: {}", c),
            }
        }
        value
    }

    fn col(&self) -> u32 {
        let mut value = 0u32;
        for c in self.seat_bsp[7..].chars() {
            value <<= 1;
            match c {
                'L' => value |= 0,
                'R' => value |= 1,
                _ => panic!("Unknown col character: {}", c),
            }
        }
        value
    }

    fn seat_id(&self) -> u32 {
        self.row() * 8 + self.col()
    }
}

fn find_my_seat_id(passes: &[BoardingPass]) -> u32 {
    let mut seat_ids: Vec<u32> = passes.iter().map(|pass| pass.seat_id()).collect();
    seat_ids.sort_unstable();

    let mut last_seat_id: Option<u32> = None;
    for seat_id in seat_ids {
        if let Some(last_id) = last_seat_id {
            if seat_id == last_id + 2 {
                return last_id + 1;
            }
        }
        last_seat_id = Some(seat_id);
    }

    panic!("Unable to find my seat!");
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<BoardingPass> {
    input.lines().map(BoardingPass::from_string).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[BoardingPass]) -> u32 {
    let highest_seat_id = input.iter().map(|pass| pass.seat_id()).max().unwrap();
    println!("Highest seat ID: {}", highest_seat_id);
    assert_eq!(highest_seat_id, 974);
    highest_seat_id
}

#[aoc(day5, part2)]
pub fn part2(input: &[BoardingPass]) -> u32 {
    let my_seat_id = find_my_seat_id(input);
    println!("My seat ID: {}", my_seat_id);
    assert_eq!(my_seat_id, 646);
    my_seat_id
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT: &str = "FBFBBFFRLR";

    #[test]
    fn test_pass() {
        let pass = BoardingPass::from_string(EXAMPLE_INPUT);
        assert_eq!(pass.row(), 44);
        assert_eq!(pass.col(), 5);
        assert_eq!(pass.seat_id(), 357);
    }
}
