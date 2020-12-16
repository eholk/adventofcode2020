use regex::Regex;
use std::borrow::Borrow;
use std::collections::BTreeMap;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let program = parse_program(input.lines().map(|line| line.unwrap()));

    let mut machine = Machine::new();
    machine.execute_program(program.iter());

    let sum = machine.sum_memory();
    println!("Part 1: {}", sum);

    Ok(())
}

fn parse_program<T: Iterator>(lines: T) -> Vec<Instruction>
where
    T::Item: Borrow<str>,
{
    lines.map(|line| parse_instruction(line.borrow())).collect()
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Mask(String),
    Mem(u64, u64),
}

struct Machine {
    mask: (u64, u64),
    mem: BTreeMap<u64, u64>,
}

impl Machine {
    fn new() -> Machine {
        Machine {
            mask: (0, 0),
            mem: BTreeMap::new(),
        }
    }

    fn execute(&mut self, i: &Instruction) {
        match i {
            Instruction::Mask(mask) => self.mask = parse_mask(mask),
            Instruction::Mem(addr, value) => {
                self.mem.insert(*addr, apply_mask(self.mask, *value));
            }
        }
    }

    fn execute_program<'a, I: std::iter::Iterator<Item = &'a Instruction>>(&mut self, program: I) {
        for i in program {
            self.execute(&i);
        }
    }

    fn sum_memory(&self) -> u64 {
        self.mem.values().sum()
    }
}

// returns a tuple of the 1 mask and 0 mask
fn parse_mask(mask: &str) -> (u64, u64) {
    mask.chars()
        .map(|c| match c {
            'X' => (0, 1),
            '1' => (1, 1),
            '0' => (0, 0),
            _ => panic!("illegal mask: {}", mask),
        })
        .fold((0, 0), |(one_mask, zero_mask), (one, zero)| {
            ((one_mask << 1) | one, (zero_mask << 1) | zero)
        })
}

fn parse_instruction(s: &str) -> Instruction {
    lazy_static! {
        static ref MASK: Regex = Regex::new(r"mask = (?P<mask>[X10]+)").unwrap();
        static ref MEM: Regex = Regex::new(r"mem\[(?P<address>\d+)\] = (?P<value>\d+)").unwrap();
    }

    if let Some(mask) = MASK.captures(s) {
        Instruction::Mask(mask["mask"].into())
    } else if let Some(mem) = MEM.captures(s) {
        Instruction::Mem(
            mem["address"].parse().unwrap(),
            mem["value"].parse().unwrap(),
        )
    } else {
        panic!("illegal instruction: {}", s);
    }
}

fn apply_mask((ones, zeros): (u64, u64), value: u64) -> u64 {
    (value | ones) & zeros
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mask_parse() {
        assert_eq!(parse_mask("XX1X0X"), (0b001000, 0b111101));
        assert_eq!(
            parse_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            (
                0b000000000000000000000000000001000000,
                0b111111111111111111111111111111111101
            )
        );
    }

    #[test]
    fn instruction_parse() {
        assert_eq!(
            parse_instruction("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            Instruction::Mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".into())
        );
        assert_eq!(parse_instruction("mem[7] = 101"), Instruction::Mem(7, 101));
    }

    #[test]
    fn exec_program() {
        let program = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11
            mem[7] = 101
            mem[8] = 0";
        let program = parse_program(program.lines());

        let mut machine = Machine::new();

        machine.execute_program(program.iter());

        assert_eq!(machine.sum_memory(), 165);
    }

    #[test]
    fn exec_short_program() {
        let program = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            mem[8] = 11";
        let program = parse_program(program.lines());

        let mut machine = Machine::new();

        machine.execute_program(program.iter());

        assert_eq!(machine.sum_memory(), 73);
    }

    #[test]
    fn test_apply_mask() {
        assert_eq!(
            apply_mask(
                (
                    0b000000000000000000000000000001000000,
                    0b111111111111111111111111111111111101
                ),
                11
            ),
            73
        );
    }
}
