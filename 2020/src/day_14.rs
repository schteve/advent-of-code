/*
    --- Day 14: Docking Data ---
    As your ferry approaches the sea port, the captain asks for your help again. The computer system that runs this port isn't compatible with the docking program on the ferry, so the docking parameters aren't being correctly initialized in the docking program's memory.

    After a brief inspection, you discover that the sea port's computer system uses a strange bitmask system in its initialization program. Although you don't have the correct decoder chip handy, you can emulate it in software!

    The initialization program (your puzzle input) can either update the bitmask or write a value to memory. Values and memory addresses are both 36-bit unsigned integers. For example, ignoring bitmasks for a moment, a line like mem[8] = 11 would write the value 11 to memory address 8.

    The bitmask is always given as a string of 36 bits, written with the most significant bit (representing 2^35) on the left and the least significant bit (2^0, that is, the 1s bit) on the right. The current bitmask is applied to values immediately before they are written to memory: a 0 or 1 overwrites the corresponding bit in the value, while an X leaves the bit in the value unchanged.

    For example, consider the following program:

    mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    mem[8] = 11
    mem[7] = 101
    mem[8] = 0
    This program starts by specifying a bitmask (mask = ....). The mask it specifies will overwrite two bits in every written value: the 2s bit is overwritten with 0, and the 64s bit is overwritten with 1.

    The program then attempts to write the value 11 to memory address 8. By expanding everything out to individual bits, the mask is applied as follows:

    value:  000000000000000000000000000000001011  (decimal 11)
    mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    result: 000000000000000000000000000001001001  (decimal 73)
    So, because of the mask, the value 73 is written to memory address 8 instead. Then, the program tries to write 101 to address 7:

    value:  000000000000000000000000000001100101  (decimal 101)
    mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    result: 000000000000000000000000000001100101  (decimal 101)
    This time, the mask has no effect, as the bits it overwrote were already the values the mask tried to set. Finally, the program tries to write 0 to address 8:

    value:  000000000000000000000000000000000000  (decimal 0)
    mask:   XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
    result: 000000000000000000000000000001000000  (decimal 64)
    64 is written to address 8 instead, overwriting the value that was there previously.

    To initialize your ferry's docking program, you need the sum of all values left in memory after the initialization program completes. (The entire 36-bit address space begins initialized to the value 0 at every address.) In the above example, only two values in memory are not zero - 101 (at address 7) and 64 (at address 8) - producing a sum of 165.

    Execute the initialization program. What is the sum of all values left in memory after it completes?

    --- Part Two ---
    For some reason, the sea port's computer system still can't communicate with your ferry's docking program. It must be using version 2 of the decoder chip!

    A version 2 decoder chip doesn't modify the values being written at all. Instead, it acts as a memory address decoder. Immediately before a value is written to memory, each bit in the bitmask modifies the corresponding bit of the destination memory address in the following way:

    If the bitmask bit is 0, the corresponding memory address bit is unchanged.
    If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
    If the bitmask bit is X, the corresponding memory address bit is floating.
    A floating bit is not connected to anything and instead fluctuates unpredictably. In practice, this means the floating bits will take on all possible values, potentially causing many memory addresses to be written all at once!

    For example, consider the following program:

    mask = 000000000000000000000000000000X1001X
    mem[42] = 100
    mask = 00000000000000000000000000000000X0XX
    mem[26] = 1
    When this program goes to write to memory address 42, it first applies the bitmask:

    address: 000000000000000000000000000000101010  (decimal 42)
    mask:    000000000000000000000000000000X1001X
    result:  000000000000000000000000000000X1101X
    After applying the mask, four bits are overwritten, three of which are different, and two of which are floating. Floating bits take on every possible combination of values; with two floating bits, four actual memory addresses are written:

    000000000000000000000000000000011010  (decimal 26)
    000000000000000000000000000000011011  (decimal 27)
    000000000000000000000000000000111010  (decimal 58)
    000000000000000000000000000000111011  (decimal 59)
    Next, the program is about to write to memory address 26 with a different bitmask:

    address: 000000000000000000000000000000011010  (decimal 26)
    mask:    00000000000000000000000000000000X0XX
    result:  00000000000000000000000000000001X0XX
    This results in an address with three floating bits, causing writes to eight memory addresses:

    000000000000000000000000000000010000  (decimal 16)
    000000000000000000000000000000010001  (decimal 17)
    000000000000000000000000000000010010  (decimal 18)
    000000000000000000000000000000010011  (decimal 19)
    000000000000000000000000000000011000  (decimal 24)
    000000000000000000000000000000011001  (decimal 25)
    000000000000000000000000000000011010  (decimal 26)
    000000000000000000000000000000011011  (decimal 27)
    The entire 36-bit address space still begins initialized to the value 0 at every address, and you still need the sum of all values left in memory at the end of the program. In this example, the sum is 208.

    Execute the initialization program using an emulator for a version 2 decoder chip. What is the sum of all values left in memory after it completes?
*/

use crate::common::{trim_start, unsigned};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::map,
    multi::{fold_many1, many1},
    sequence::{preceded, separated_pair},
    IResult,
};
use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Mode {
    M1,
    M2,
}

#[derive(Clone, Copy)]
pub struct Mask {
    set: u64,
    clear: u64,
    floating: u64,
}

impl Mask {
    fn new() -> Self {
        Self {
            set: 0,
            clear: 0,
            floating: 0,
        }
    }

    fn from_string(input: &str) -> Self {
        Self::parser(input).unwrap().1
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, mask) = trim_start(fold_many1(one_of("01X"), Mask::new(), |mut acc, bit| {
            acc.set <<= 1;
            acc.clear <<= 1;
            acc.floating <<= 1;
            match bit {
                '0' => acc.clear |= 1,
                '1' => acc.set |= 1,
                'X' => acc.floating |= 1,
                _ => panic!("Invalid bit character received!"),
            }
            acc
        }))(input)?;

        assert!(mask.is_valid());

        Ok((input, mask))
    }

    fn is_valid(&self) -> bool {
        (self.set & self.clear) == 0
            && (self.set & self.floating) == 0
            && (self.clear & self.floating) == 0
    }

    fn apply_value(&self, value: u64) -> u64 {
        assert!(self.is_valid());

        let mut tmp = value;
        tmp |= self.set;
        tmp &= !self.clear;
        tmp
    }

    fn apply_address(&self, addr: usize) -> MaskAddrIterator {
        MaskAddrIterator {
            mask: &self,
            addr,
            count: 0,
        }
    }
}

impl std::fmt::Display for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for idx in 0..36 {
            let digit = 63 - idx;
            let mask = 1 << digit;
            if self.clear & mask != 0 {
                write!(f, "0")?;
            } else if self.set & mask != 0 {
                write!(f, "1")?;
            } else if self.floating & mask != 0 {
                write!(f, "X")?;
            }
        }
        Ok(())
    }
}

struct MaskAddrIterator<'a> {
    mask: &'a Mask,
    addr: usize,
    count: u64,
}

impl<'a> Iterator for MaskAddrIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let mut addr = self.addr | self.mask.set as usize;
        let mut count = self.count; // This is modified as we loop to know if all bits have been used (if not, this iterator is complete)
        let mut count_idx = 0;
        for float_idx in 0..36 {
            if self.mask.floating & (1 << float_idx) != 0 {
                // Set the digit in float to the equivalent digit in count
                if self.count & (1 << count_idx) == 0 {
                    addr &= !(1 << float_idx);
                } else {
                    addr |= 1 << float_idx;
                }
                count &= !(1 << count_idx); // This digit has been used, clear it
                count_idx += 1;
            }
        }

        // If count has any digits that haven't been allocated, then it has exceeded the number of floating bits available and thus this iterator is complete
        if count == 0 {
            self.count += 1;
            Some(addr)
        } else {
            None
        }
    }
}

pub enum Instruction {
    UpdateMask(Mask),
    WriteMemory(usize, u64),
}

impl Instruction {
    fn parser(input: &str) -> IResult<&str, Self> {
        let (input, instruction) = alt((
            preceded(
                trim_start(tag("mask = ")),
                map(Mask::parser, Self::UpdateMask),
            ),
            preceded(
                trim_start(tag("mem[")),
                map(separated_pair(unsigned, tag("] = "), unsigned), |mem| {
                    Self::WriteMemory(mem.0, mem.1)
                }),
            ),
        ))(input)?;

        Ok((input, instruction))
    }
}

struct Computer {
    mask: Mask,
    memory: HashMap<usize, u64>,
}

impl Computer {
    fn new() -> Self {
        Self {
            mask: Mask::new(),
            memory: HashMap::new(),
        }
    }

    fn execute(&mut self, instruction: &Instruction, mode: Mode) {
        match instruction {
            Instruction::UpdateMask(mask) => self.mask = *mask,
            Instruction::WriteMemory(addr, value) => match mode {
                Mode::M1 => {
                    self.memory.insert(*addr, self.mask.apply_value(*value));
                }
                Mode::M2 => {
                    for a in self.mask.apply_address(*addr) {
                        self.memory.insert(a, *value);
                    }
                }
            },
        }
    }

    fn execute_all(&mut self, instructions: &[Instruction], mode: Mode) {
        instructions.iter().for_each(|i| self.execute(i, mode));
    }

    fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    many1(Instruction::parser)(input).unwrap().1
}

#[aoc(day14, part1)]
pub fn part1(input: &[Instruction]) -> u64 {
    let mut computer = Computer::new();
    computer.execute_all(input, Mode::M1);
    let memory_sum = computer.memory_sum();
    assert_eq!(memory_sum, 7997531787333);
    memory_sum
}

#[aoc(day14, part2)]
pub fn part2(input: &[Instruction]) -> u64 {
    let mut computer = Computer::new();
    computer.execute_all(input, Mode::M2);
    let memory_sum = computer.memory_sum();
    assert_eq!(memory_sum, 3564822193820);
    memory_sum
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE_INPUT1: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    static EXAMPLE_INPUT2: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_mask_apply_value() {
        let mask = Mask::from_string("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!(mask.apply_value(11), 73);
        assert_eq!(mask.apply_value(101), 101);
        assert_eq!(mask.apply_value(0), 64);
    }

    #[test]
    fn test_mask_apply_address() {
        let mask = Mask::from_string("000000000000000000000000000000X1001X");
        let mut mask_addr_iter = mask.apply_address(42);
        assert_eq!(mask_addr_iter.next(), Some(26));
        assert_eq!(mask_addr_iter.next(), Some(27));
        assert_eq!(mask_addr_iter.next(), Some(58));
        assert_eq!(mask_addr_iter.next(), Some(59));
        assert_eq!(mask_addr_iter.next(), None);

        let mask = Mask::from_string("00000000000000000000000000000000X0XX");
        let mut mask_addr_iter = mask.apply_address(26);
        assert_eq!(mask_addr_iter.next(), Some(16));
        assert_eq!(mask_addr_iter.next(), Some(17));
        assert_eq!(mask_addr_iter.next(), Some(18));
        assert_eq!(mask_addr_iter.next(), Some(19));
        assert_eq!(mask_addr_iter.next(), Some(24));
        assert_eq!(mask_addr_iter.next(), Some(25));
        assert_eq!(mask_addr_iter.next(), Some(26));
        assert_eq!(mask_addr_iter.next(), Some(27));
        assert_eq!(mask_addr_iter.next(), None);
    }

    #[test]
    fn test_memory_sum() {
        let mut computer = Computer::new();
        let instructions = input_generator(EXAMPLE_INPUT1);
        computer.execute_all(&instructions, Mode::M1);
        assert_eq!(computer.memory_sum(), 165);

        let mut computer = Computer::new();
        let instructions = input_generator(EXAMPLE_INPUT2);
        computer.execute_all(&instructions, Mode::M2);
        assert_eq!(computer.memory_sum(), 208);
    }
}
