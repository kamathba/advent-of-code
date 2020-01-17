use ::std::*;
/**
    --- Day 5: Sunny with a Chance of Asteroids ---
    You're starting to sweat as the ship makes its way toward Mercury. The Elves suggest that you get the air conditioner working by upgrading your ship computer to support the Thermal Environment Supervision Terminal.

    The Thermal Environment Supervision Terminal (TEST) starts by running a diagnostic program (your puzzle input). The TEST diagnostic program will run on your existing Intcode computer after a few modifications:

    First, you'll need to add two new instructions:

    Opcode 3 takes a single integer as input and saves it to the position given by its only parameter. For example, the instruction 3,50 would take an input value and store it at address 50.
    Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.
    Programs that use these instructions will come with documentation that explains what should be connected to the input and output. The program 3,0,4,0,99 outputs whatever it gets as input, then halts.

    Second, you'll need to add support for parameter modes:

    Each parameter of an instruction is handled based on its parameter mode. Right now, your ship computer already understands parameter mode 0, position mode, which causes the parameter to be interpreted as a position - if the parameter is 50, its value is the value stored at address 50 in memory. Until now, all parameters have been in position mode.

    Now, your ship computer will also need to handle parameters in mode 1, immediate mode. In immediate mode, a parameter is interpreted as a value - if the parameter is 50, its value is simply 50.

    Parameter modes are stored in the same value as the instruction's opcode. The opcode is a two-digit number based only on the ones and tens digit of the value, that is, the opcode is the rightmost two digits of the first value in an instruction. Parameter modes are single digits, one per parameter, read right-to-left from the opcode: the first parameter's mode is in the hundreds digit, the second parameter's mode is in the thousands digit, the third parameter's mode is in the ten-thousands digit, and so on. Any missing modes are 0.

    For example, consider the program 1002,4,3,4,33.

    The first instruction, 1002,4,3,4, is a multiply instruction - the rightmost two digits of the first value, 02, indicate opcode 2, multiplication. Then, going right to left, the parameter modes are 0 (hundreds digit), 1 (thousands digit), and 0 (ten-thousands digit, not present and therefore zero):

    ABCDE
     1002

    DE - two-digit opcode,      02 == opcode 2
     C - mode of 1st parameter,  0 == position mode
     B - mode of 2nd parameter,  1 == immediate mode
     A - mode of 3rd parameter,  0 == position mode,
                                      omitted due to being a leading zero
    This instruction multiplies its first two parameters. The first parameter, 4 in position mode, works like it did before - its value is the value stored at address 4 (33). The second parameter, 3 in immediate mode, simply has value 3. The result of this operation, 33 * 3 = 99, is written according to the third parameter, 4 in position mode, which also works like it did before - 99 is written to address 4.

    Parameters that an instruction writes to will never be in immediate mode.

    Finally, some notes:

    It is important to remember that the instruction pointer should increase by the number of values in the instruction after the instruction finishes. Because of the new instructions, this amount is no longer always 4.
    Integers can be negative: 1101,100,-1,4,0 is a valid program (find 100 + -1, store the result in position 4).
    The TEST diagnostic program will start by requesting from the user the ID of the system to test by running an input instruction - provide it 1, the ID for the ship's air conditioner unit.

    It will then perform a series of diagnostic tests confirming that various parts of the Intcode computer, like parameter modes, function correctly. For each test, it will run an output instruction indicating how far the result of the test was from the expected value, where 0 means the test was successful. Non-zero outputs mean that a function is not working correctly; check the instructions that were run before the output instruction to see which one failed.

    Finally, the program will output a diagnostic code and immediately halt. This final output isn't an error; an output followed immediately by a halt means the program finished. If all outputs were zero except the diagnostic code, the diagnostic program ran successfully.

    After providing 1 to the only input instruction and passing all the tests, what diagnostic code does the program produce?
*/
use num_derive::FromPrimitive;
use num_traits::{pow, FromPrimitive};
use std::iter::FromIterator;

#[derive(FromPrimitive)]
enum OpCode {
    Add = 1,
    Multiply = 2,
    Input = 3,
    Output = 4,
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

fn execute_program(ro_program: &[i32]) -> i32 {
    let mut program: Vec<i32> = Vec::from_iter(ro_program.iter().cloned());
    let mut pc: usize = 0;

    loop {
        let opcode = program[pc] % 100;
        let parameter_mode = program[pc] / 100;

        match FromPrimitive::from_i32(opcode) {
            Some(OpCode::End) => break program[0],
            Some(OpCode::Input) => {
                let r1 = program[pc + 1];
                let mut ret = String::new();
                io::stdin()
                    .read_line(&mut ret)
                    .expect("Failed to read from stdin");
                program[r1 as usize] = ret.trim().parse::<i32>().expect("Not an integer");
                pc += 2;
            }
            Some(OpCode::Output) => {
                let r1 = get_reg(&program, pc, parameter_mode, 1);
                println!("{}", r1);
                pc += 2;
            }
            Some(x) => {
                let target: usize = program[pc + 3] as usize;
                let r1 = get_reg(&program, pc, parameter_mode, 1);
                let r2 = get_reg(&program, pc, parameter_mode, 2);

                program[target] = match x {
                    OpCode::Add => r1 + r2,
                    OpCode::Multiply => r1 * r2,
                    _ => 0,
                };

                pc += 4;
            }
            None => {
                println!("Segfault");
                break 0;
            }
        };
    }
}

fn main() -> std::io::Result<()> {
    let ro_program: Vec<i32> = include_str!("../input")
        .split(',')
        .map(|s| s.parse::<i32>().expect("Not an unsigned integer"))
        .collect();

    execute_program(&ro_program);

    Ok(())
}
