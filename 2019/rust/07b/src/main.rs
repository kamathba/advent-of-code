use ::std::*;
/**
    --- Part Two ---
    It's no good - in this configuration, the amplifiers can't generate a large enough output signal to produce the thrust you'll need. The Elves quickly talk you through rewiring the amplifiers into a feedback loop:

          O-------O  O-------O  O-------O  O-------O  O-------O
    0 -+->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-.
       |  O-------O  O-------O  O-------O  O-------O  O-------O |
       |                                                        |
       '--------------------------------------------------------+
                                                                |
                                                                v
                                                         (to thrusters)
    Most of the amplifiers are connected as they were before; amplifier A's output is connected to amplifier B's input, and so on. However, the output from amplifier E is now connected into amplifier A's input. This creates the feedback loop: the signal will be sent through the amplifiers many times.

    In feedback loop mode, the amplifiers need totally different phase settings: integers from 5 to 9, again each used exactly once. These settings will cause the Amplifier Controller Software to repeatedly take input and produce output many times before halting. Provide each amplifier its phase setting at its first input instruction; all further input/output instructions are for signals.

    Don't restart the Amplifier Controller Software on any amplifier during this process. Each one should continue receiving and sending signals until it halts.

    All signals sent or received in this process will be between pairs of amplifiers except the very first signal and the very last signal. To start the process, a 0 signal is sent to amplifier A's input exactly once.

    Eventually, the software on the amplifiers will halt after they have processed the final loop. When this happens, the last output signal from amplifier E is sent to the thrusters. Your job is to find the largest output signal that can be sent to the thrusters using the new phase settings and feedback loop arrangement.

    Here are some example programs:

    Max thruster signal 139629729 (from phase setting sequence 9,8,7,6,5):

    3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
    27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5
    Max thruster signal 18216 (from phase setting sequence 9,7,8,5,6):

    3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
    -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
    53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10
    Try every combination of the new phase settings on the amplifier feedback loop. What is the highest signal that can be sent to the thrusters?
*/
use num_derive::FromPrimitive;
use num_traits::{pow, FromPrimitive};
use permute::permutations_of;
use std::iter::FromIterator;

#[derive(FromPrimitive, PartialEq)]
enum OpCode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfNot = 6,
    LessThan = 7,
    Equals = 8,
    End = 99,
}

#[derive(Eq, PartialEq)]
enum ProgramState {
    NeedsInput,
    Finished,
}

#[derive(Debug)]
struct Program {
    exe: Vec<i32>,
    pc: usize,
    pub input: Option<i32>,
    pub output: Option<i32>,
}

impl Program {
    fn new(ro_program: &[i32]) -> Program {
        Program {
            exe: Vec::from_iter(ro_program.iter().cloned()),
            pc: 0,
            input: None,
            output: None,
        }
    }

    fn parameter(&self, position: usize) -> i32 {
        self.exe[self.pc + position]
    }

    fn get_reg(&self, position: usize) -> i32 {
        let parameter_mode = self.exe[self.pc] / 100;
        let val = self.parameter(position);
        let digit = pow(10, position - 1);

        if (parameter_mode / digit) % 10 == 1 {
            val
        } else {
            self.exe[val as usize]
        }
    }

    fn terminated(&self) -> bool {
        self.parameter(0) == OpCode::End as i32
    }
    fn execute(&mut self) -> ProgramState {
        loop {
            let opcode = FromPrimitive::from_i32(self.parameter(0) % 100).expect("Segfault");

            match opcode {
                OpCode::End => {
                    return ProgramState::Finished;
                }
                OpCode::Add | OpCode::Multiply => {
                    let target: usize = self.parameter(3) as usize;
                    let r1 = self.get_reg(1);
                    let r2 = self.get_reg(2);

                    self.exe[target] = match opcode {
                        OpCode::Add => r1 + r2,
                        OpCode::Multiply => r1 * r2,
                        _ => unreachable!(),
                    };

                    self.pc += 4;
                }
                OpCode::Input => {
                    if let Some(input) = self.input {
                        let r1 = self.parameter(1);
                        self.exe[r1 as usize] = input;
                        self.pc += 2;
                        self.input = None;
                    } else {
                        return ProgramState::NeedsInput;
                    }
                }
                OpCode::Output => {
                    let r1 = self.get_reg(1);
                    self.output = Some(r1);
                    self.pc += 2;
                }
                OpCode::JumpIfTrue | OpCode::JumpIfNot => {
                    let r1 = self.get_reg(1);
                    let r2 = self.get_reg(2);
                    let condition = r1 != 0;

                    self.pc = if (opcode == OpCode::JumpIfTrue) == condition {
                        r2 as usize
                    } else {
                        self.pc + 3
                    }
                }
                OpCode::LessThan | OpCode::Equals => {
                    let target: usize = self.parameter(3) as usize;
                    let r1 = self.get_reg(1);
                    let r2 = self.get_reg(2);

                    self.exe[target] = match opcode {
                        OpCode::LessThan => (r1 < r2) as i32,
                        OpCode::Equals => (r1 == r2) as i32,
                        _ => unreachable!(),
                    };

                    self.pc += 4;
                }
            };
        }
    }
}

fn feedback_thruster_output(ro_program: &[i32], sequence: &[i32]) -> i32 {
    assert!(sequence.len() == 5);

    let mut programs: Vec<Program> = Vec::new();

    for _i in 0..5 {
        programs.push(Program::new(&ro_program));
    }

    let mut codes = Vec::from_iter(sequence.iter().cloned());
    codes.push(0);

    loop {
        for program in programs.iter_mut() {
            if program.execute() == ProgramState::NeedsInput && !codes.is_empty() {
                let x = codes.remove(0);
                program.input = Some(x);
            }

            if let Some(x) = program.output {
                codes.push(x);
                program.output = None;
            }
        }

        programs = programs
            .into_iter()
            .filter(|x| !x.terminated() || x.output.is_some())
            .collect::<Vec<_>>();

        if programs.is_empty() {
            break;
        }
    }

    assert!(codes.len() == 1);
    codes.pop().expect("Program failed")
}

fn main() -> std::io::Result<()> {
    let program: Vec<i32> = include_str!("../input")
        .split(',')
        .map(|s| s.parse::<i32>().expect("Not an integer"))
        .collect();

    let output = permutations_of(&[5, 6, 7, 8, 9])
        .map(|x| x.map(|&x| x as i32).collect::<Vec<i32>>())
        .max_by_key(|x| feedback_thruster_output(&program, &x))
        .expect("failed");

    println!(
        "Max thruster code: {:?} = {}",
        output,
        feedback_thruster_output(&program, &output)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::feedback_thruster_output;

    #[test]
    fn example_program1() {
        let program = [
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        let sequence = [9, 8, 7, 6, 5];
        assert_eq!(139629729, feedback_thruster_output(&program, &sequence));
    }

    #[test]
    fn example_program2() {
        let program = [
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        let sequence = [9, 7, 8, 5, 6];
        assert_eq!(18216, feedback_thruster_output(&program, &sequence));
    }
}
