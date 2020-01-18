use ::std::*;
/**
    --- Part Two ---
    The air conditioner comes online! Its cold air feels good for a while, but then the TEST alarms start to go off. Since the air conditioner can't vent its heat anywhere but back into the spacecraft, it's actually making the air inside the ship warmer.

    Instead, you'll need to use the TEST to extend the thermal radiators. Fortunately, the diagnostic program (your puzzle input) is already equipped for this. Unfortunately, your Intcode computer is not.

    Your computer is only missing a few opcodes:

    Opcode 5 is jump-if-true: if the first parameter is non-zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
    Opcode 6 is jump-if-false: if the first parameter is zero, it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
    Opcode 7 is less than: if the first parameter is less than the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    Opcode 8 is equals: if the first parameter is equal to the second parameter, it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
    Like all instructions, these instructions need to support parameter modes as described above.

    Normally, after an instruction is finished, the instruction pointer increases by the number of values in that instruction. However, if the instruction modifies the instruction pointer, that value is used and the instruction pointer is not automatically increased.

    For example, here are several programs that take one input, compare it to the value 8, and then produce one output:

    3,9,8,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    3,9,7,9,10,9,4,9,99,-1,8 - Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    3,3,1108,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    3,3,1107,-1,8,3,4,3,99 - Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    Here are some jump tests that take an input, then output 0 if the input was zero or 1 if the input was non-zero:

    3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9 (using position mode)
    3,3,1105,-1,9,1101,0,0,12,4,12,99,1 (using immediate mode)
    Here's a larger example:

    3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
    1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
    999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
    The above example program uses an input instruction to ask for a single number. The program will then output 999 if the input value is below 8, output 1000 if the input value is equal to 8, or output 1001 if the input value is greater than 8.

    This time, when the TEST diagnostic program runs its input instruction to get the ID of the system to test, provide it 5, the ID for the ship's thermal radiator controller. This diagnostic test suite only outputs one number, the diagnostic code.

    What is the diagnostic code for system ID 5?
*/
use num_derive::FromPrimitive;
use num_traits::{pow, FromPrimitive};
use std::string::String;

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
    fn input(&mut self) -> String;
    fn output(&mut self, val: i32);
}

struct CommandLineTerminal {}

impl Terminal for CommandLineTerminal {
    fn input(&mut self) -> String {
        let mut val = String::new();
        io::stdin()
            .read_line(&mut val)
            .expect("Failed to read from stdin");

        val
    }

    fn output(&mut self, val: i32) {
        print!("{}", val);
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
                let ret = term.input();

                program[r1 as usize] = ret.trim().parse::<i32>().expect("Not an integer");
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

fn main() -> std::io::Result<()> {
    let mut program: Vec<i32> = include_str!("../input")
        .split(',')
        .map(|s| s.parse::<i32>().expect("Not an integer"))
        .collect();

    let mut terminal = CommandLineTerminal {};

    execute_program(&mut terminal, &mut program);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{execute_program, Terminal};

    #[derive(Default)]
    struct TestTerminal {
        pub inputs: Vec<String>,
        pub outputs: Vec<i32>,
    }

    impl Terminal for TestTerminal {
        fn input(&mut self) -> String {
            self.inputs.remove(0)
        }

        fn output(&mut self, val: i32) {
            self.outputs.push(val);
        }
    }

    #[test]
    fn multiply_program() {
        let mut program: [i32; 5] = [1002, 4, 3, 4, 33];
        let mut test_terminal = TestTerminal::default();
        execute_program(&mut test_terminal, &mut program);

        assert_eq!(program[4], 99);
    }

    fn execute_with_input(term: &mut TestTerminal, ro_program: &[i32], input: &str) {
        use std::iter::FromIterator;

        let mut program = Vec::from_iter(ro_program.iter().cloned());
        term.inputs.push(String::from(input));
        execute_program(&mut *term, &mut program);
    }

    #[test]
    fn compare_program() {
        let mut test_terminal = TestTerminal::default();

        let equal_program = [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        execute_with_input(&mut test_terminal, &equal_program, "8\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 1);

        execute_with_input(&mut test_terminal, &equal_program, "101\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 0);

        let lessthan_program = [3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        execute_with_input(&mut test_terminal, &lessthan_program, "-100\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 1);

        execute_with_input(&mut test_terminal, &lessthan_program, "9\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 0);

        let immediateequal_program = [3, 3, 1108, -1, 8, 3, 4, 3, 99];
        execute_with_input(&mut test_terminal, &immediateequal_program, "8\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 1);

        execute_with_input(&mut test_terminal, &immediateequal_program, "7\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 0);

        let immediatelessthan_program = [3, 3, 1107, -1, 8, 3, 4, 3, 99];
        execute_with_input(&mut test_terminal, &immediatelessthan_program, "-100\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 1);

        execute_with_input(&mut test_terminal, &immediatelessthan_program, "9\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 0);
    }

    #[test]
    fn jump_program() {
        let mut test_terminal = TestTerminal::default();
        let ro_program = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];

        execute_with_input(&mut test_terminal, &ro_program, "0\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 999);

        execute_with_input(&mut test_terminal, &ro_program, "8\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 1000);

        execute_with_input(&mut test_terminal, &ro_program, "9\n");
        assert_eq!(test_terminal.outputs.pop().expect("No outputs"), 1001);
    }
}
