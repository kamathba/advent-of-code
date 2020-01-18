use ::std::*;
/**
    --- Day 7: Amplification Circuit ---
    Based on the navigational maps, you're going to need to send more power to your ship's thrusters to reach Santa in time. To do this, you'll need to configure a series of amplifiers already installed on the ship.

    There are five amplifiers connected in series; each one receives an input signal and produces an output signal. They are connected such that the first amplifier's output leads to the second amplifier's input, the second amplifier's output leads to the third amplifier's input, and so on. The first amplifier's input value is 0, and the last amplifier's output leads to your ship's thrusters.

        O-------O  O-------O  O-------O  O-------O  O-------O
    0 ->| Amp A |->| Amp B |->| Amp C |->| Amp D |->| Amp E |-> (to thrusters)
        O-------O  O-------O  O-------O  O-------O  O-------O
    The Elves have sent you some Amplifier Controller Software (your puzzle input), a program that should run on your existing Intcode computer. Each amplifier will need to run a copy of the program.

    When a copy of the program starts running on an amplifier, it will first use an input instruction to ask the amplifier for its current phase setting (an integer from 0 to 4). Each phase setting is used exactly once, but the Elves can't remember which amplifier needs which phase setting.

    The program will then call another input instruction to get the amplifier's input signal, compute the correct output signal, and supply it back to the amplifier with an output instruction. (If the amplifier has not yet received an input signal, it waits until one arrives.)

    Your job is to find the largest output signal that can be sent to the thrusters by trying every possible combination of phase settings on the amplifiers. Make sure that memory is not shared or reused between copies of the program.

    For example, suppose you want to try the phase setting sequence 3,1,2,4,0, which would mean setting amplifier A to phase setting 3, amplifier B to setting 1, C to 2, D to 4, and E to 0. Then, you could determine the output signal that gets sent from amplifier E to the thrusters with the following steps:

    Start the copy of the amplifier controller software that will run on amplifier A. At its first input instruction, provide it the amplifier's phase setting, 3. At its second input instruction, provide it the input signal, 0. After some calculations, it will use an output instruction to indicate the amplifier's output signal.
    Start the software for amplifier B. Provide it the phase setting (1) and then whatever output signal was produced from amplifier A. It will then produce a new output signal destined for amplifier C.
    Start the software for amplifier C, provide the phase setting (2) and the value from amplifier B, then collect its output signal.
    Run amplifier D's software, provide the phase setting (4) and input value, and collect its output signal.
    Run amplifier E's software, provide the phase setting (0) and input value, and collect its output signal.
    The final output signal from amplifier E would be sent to the thrusters. However, this phase setting sequence may not have been the best one; another sequence might have sent a higher signal to the thrusters.

    Here are some example programs:

    Max thruster signal 43210 (from phase setting sequence 4,3,2,1,0):

    3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0
    Max thruster signal 54321 (from phase setting sequence 0,1,2,3,4):

    3,23,3,24,1002,24,10,24,1002,23,-1,23,
    101,5,23,23,1,24,23,23,4,23,99,0,0
    Max thruster signal 65210 (from phase setting sequence 1,0,4,3,2):

    3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
    1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0
    Try every combination of phase settings on the amplifiers. What is the highest signal that can be sent to the thrusters?
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

fn get_reg(ro_program: &[i32], pc: usize, parameter_mode: i32, position: usize) -> i32 {
    let val = ro_program[pc + position];
    let digit = pow(10, position - 1);

    if (parameter_mode / digit) % 10 == 1 {
        val
    } else {
        ro_program[val as usize]
    }
}

trait Terminal {
    fn input(&mut self) -> i32;
    fn output(&mut self, val: i32);
}

#[derive(Default)]
struct TestTerminal {
    pub inputs: Vec<i32>,
    pub outputs: Vec<i32>,
}

impl Terminal for TestTerminal {
    fn input(&mut self) -> i32 {
        self.inputs.remove(0)
    }

    fn output(&mut self, val: i32) {
        self.outputs.push(val);
    }
}

fn execute_program(term: &mut impl Terminal, program: &mut [i32]) {
    let mut pc: usize = 0;

    loop {
        let opcode = FromPrimitive::from_i32(program[pc] % 100).expect("Segfault");
        let parameter_mode = program[pc] / 100;

        match opcode {
            OpCode::End => break,
            OpCode::Add | OpCode::Multiply => {
                let target: usize = program[pc + 3] as usize;
                let r1 = get_reg(&program, pc, parameter_mode, 1);
                let r2 = get_reg(&program, pc, parameter_mode, 2);

                program[target] = match opcode {
                    OpCode::Add => r1 + r2,
                    OpCode::Multiply => r1 * r2,
                    _ => unreachable!(),
                };

                pc += 4;
            }
            OpCode::Input => {
                let r1 = program[pc + 1];
                program[r1 as usize] = term.input();
                pc += 2;
            }
            OpCode::Output => {
                let r1 = get_reg(&program, pc, parameter_mode, 1);
                term.output(r1);
                pc += 2;
            }
            OpCode::JumpIfTrue | OpCode::JumpIfNot => {
                let r1 = get_reg(&program, pc, parameter_mode, 1);
                let r2 = get_reg(&program, pc, parameter_mode, 2);
                let condition = r1 != 0;

                pc = if (opcode == OpCode::JumpIfTrue) == condition {
                    r2 as usize
                } else {
                    pc + 3
                }
            }
            OpCode::LessThan | OpCode::Equals => {
                let target: usize = program[pc + 3] as usize;
                let r1 = get_reg(&program, pc, parameter_mode, 1);
                let r2 = get_reg(&program, pc, parameter_mode, 2);

                program[target] = match opcode {
                    OpCode::LessThan => (r1 < r2) as i32,
                    OpCode::Equals => (r1 == r2) as i32,
                    _ => unreachable!(),
                };

                pc += 4;
            }
        };
    }
}

fn thruster_output(ro_program: &[i32], sequence: &[i32]) -> i32 {
    let mut terminal = TestTerminal::default();

    assert!(sequence.len() == 5);
    terminal.outputs.push(0);

    for setting in sequence {
        let mut program = Vec::from_iter(ro_program.iter().cloned());

        terminal.inputs.push(*setting);
        terminal
            .inputs
            .push(terminal.outputs.pop().expect("No output"));

        execute_program(&mut terminal, &mut program);
    }

    terminal.outputs.pop().expect("No output")
}

fn main() -> std::io::Result<()> {
    let program: Vec<i32> = include_str!("../input")
        .split(',')
        .map(|s| s.parse::<i32>().expect("Not an integer"))
        .collect();

    let output = permutations_of(&[0, 1, 2, 3, 4])
        .map(|x| x.map(|&x| x as i32).collect::<Vec<i32>>())
        .max_by_key(|x| thruster_output(&program, &x))
        .expect("failed");

    println!("Max thruster code: {:?} = {}", output, thruster_output(&program, &output));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::thruster_output;

    #[test]
    fn example_program1() {
        let program = [
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        let sequence = [4, 3, 2, 1, 0];
        assert_eq!(43210, thruster_output(&program, &sequence));
    }

    #[test]
    fn example_program2() {
        let program = [
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        let sequence = [0, 1, 2, 3, 4];
        assert_eq!(54321, thruster_output(&program, &sequence));
    }

    #[test]
    fn example_program3() {
        let program = [
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        let sequence = [1, 0, 4, 3, 2];
        assert_eq!(65210, thruster_output(&program, &sequence));
    }
}
