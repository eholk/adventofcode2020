use std::borrow::Borrow;

pub fn run<IO: std::io::BufRead>(input: IO) -> std::io::Result<()> {
    let mut machine = parse_program(input.lines().map(|line| line.unwrap()));

    println!("Part 1: {}", machine.trace());

    Ok(())
}

fn parse_program<Lines: Iterator>(lines: Lines) -> Machine
where
    Lines::Item: Borrow<str>,
{
    let code = lines.map(|line| Instruction::parse(line.borrow())).collect();
    Machine::with_program(code)
}

#[derive(PartialEq, Debug)]
struct Instruction {
    opcode: &'static str,
    argument: isize,
}

impl Instruction {
    fn parse(s: &str) -> Instruction {
        let mut parts = s.split(" ");
        let opcode = match parts.next().unwrap() {
            "nop" => "nop",
            "acc" => "acc",
            "jmp" => "jmp",
            other => panic!("Unknown opcode: {}", other),
        };
        let argument = parts.next().unwrap().parse().unwrap();
        Instruction { opcode, argument }
    }
}

struct Machine {
    code: Vec<Instruction>,
    ip: isize,
    acc: isize,
}

impl Machine {
    fn with_program(program: Vec<Instruction>) -> Machine {
        Machine {
            code: program,
            ip: 0,
            acc: 0,
        }
    }

    fn step(&mut self) {
        match &self.code[self.ip as usize] {
            Instruction { opcode: "nop", argument: _ } => self.ip += 1,
            Instruction { opcode: "acc", argument } => {
                self.acc += argument;
                self.ip += 1;
            },
            Instruction { opcode: "jmp", argument } => self.ip += argument,
            Instruction { opcode, argument: _ } => panic!("Unknown opcode: {}", opcode),
        }
    }

    fn trace(&mut self) -> isize {
        self.trace_robust(false)
    }

    fn reset(&mut self) {
        self.ip = 0;
        self.acc = 0;
    }

    fn terminated(&self) -> bool {
        self.ip as usize == self.code.len()
    }

    fn toggle_instruction(&mut self, ip: usize) {
        match self.code[ip].opcode {
            "jmp" => self.code[ip].opcode = "nop",
            "nop" => self.code[ip].opcode = "jmp",
            other => panic!("can't toggle instruction {}", other),
        }
    }

    fn trace_robust(&mut self, fix_loop: bool) -> isize {
        let mut visited = vec![false; self.code.len()];
        while !self.terminated() {
            let ip = self.ip as usize;
            if visited[ip] {
                return self.acc;
            }
            visited[ip] = true;
            self.step();
            let new_ip = self.ip as usize;
            // Check if we need to patch up the loop.
            if fix_loop && visited[new_ip] {
                // First try toggling the instruction we just ran.
                self.toggle_instruction(ip);
                self.reset();
                let result = self.trace();
                if self.terminated() {
                    // Great, that worked!
                    return result;
                }
                // Otherwise, try toggling the target instruction
                self.toggle_instruction(ip); // reset the old instruction
                self.toggle_instruction(new_ip);
                self.reset();
                let result = self.trace();
                if self.terminated() {
                    return result;
                }

                panic!("Could not patch loop");
            }
        }
        self.acc
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_numbers() {
        assert_eq!("-14".parse::<isize>(), Ok(-14));
        assert_eq!("+42".parse::<isize>(), Ok(42));
    }

    #[test]
    fn parse_instructions() {
        assert_eq!(
            Instruction::parse("nop +0"),
            Instruction {
                opcode: "nop",
                argument: 0
            }
        );

        assert_eq!(
            Instruction::parse("acc -99"),
            Instruction {
                opcode: "acc",
                argument: -99
            }
        );
    }

    #[test]
    fn example_trace() {
        let program = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let mut machine = parse_program(program.lines());
        assert_eq!(machine.trace(), 5);
    }

    #[test]
    fn fix_loop() {
        let program = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let mut machine = parse_program(program.lines());
        assert_eq!(machine.trace_robust(true), 8);
    }
}
