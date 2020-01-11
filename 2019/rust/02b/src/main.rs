/**
    --- Part Two ---
    "Good, the new computer seems to be working correctly! Keep it nearby during this mission - you'll probably use it again. Real Intcode computers support many more features than your new one, but we'll let you know what they are as you need them."

    "However, your current priority should be to complete your gravity assist around the Moon. For this mission to succeed, we should settle on some terminology for the parts you've already built."

    Intcode programs are given as a list of integers; these values are used as the initial state for the computer's memory. When you run an Intcode program, make sure to start by initializing memory to the program's values. A position in memory is called an address (for example, the first value in memory is at "address 0").

    Opcodes (like 1, 2, or 99) mark the beginning of an instruction. The values used immediately after an opcode, if any, are called the instruction's parameters. For example, in the instruction 1,2,3,4, 1 is the opcode; 2, 3, and 4 are the parameters. The instruction 99 contains only an opcode and has no parameters.

    The address of the current instruction is called the instruction pointer; it starts at 0. After an instruction finishes, the instruction pointer increases by the number of values in the instruction; until you add more instructions to the computer, this is always 4 (1 opcode + 3 parameters) for the add and multiply instructions. (The halt instruction would increase the instruction pointer by 1, but it halts the program instead.)

    "With terminology out of the way, we're ready to proceed. To complete the gravity assist, you need to determine what pair of inputs produces the output 19690720."

    The inputs should still be provided to the program by replacing the values at addresses 1 and 2, just like before. In this program, the value placed in address 1 is called the noun, and the value placed in address 2 is called the verb. Each of the two input values will be between 0 and 99, inclusive.

    Once the program has halted, its output is available at address 0, also just like before. Each time you try a pair of inputs, make sure you first reset the computer's memory to the values in the program (your puzzle input) - in other words, don't reuse memory from a previous attempt.

    Find the input noun and verb that cause the program to produce the output 19690720. What is 100 * noun + verb? (For example, if noun=12 and verb=2, the answer would be 1202.)
*/
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::iter::FromIterator;

#[derive(FromPrimitive)]
enum OpCode {
    Add = 1,
    Multiply = 2,
    End = 99,
}

fn execute_program(ro_program: &[u32], noun: u32, verb: u32) -> u32 {
    let mut program: Vec<u32> = Vec::from_iter(ro_program.iter().cloned());
    let mut pc: usize = 0;

    program[1] = noun;
    program[2] = verb;

    loop {
        match FromPrimitive::from_u32(program[pc]) {
            Some(OpCode::End) => break program[0],
            Some(x) => {
                let target: usize = program[pc + 3] as usize;
                let r1: usize = program[pc + 1] as usize;
                let r2: usize = program[pc + 2] as usize;

                program[target] = match x {
                    OpCode::Add => program[r1] + program[r2],
                    OpCode::Multiply => program[r1] * program[r2],
                    _ => 0,
                }
            }
            None => {
                println!("Segfault");
                break 0;
            }
        };

        pc += 4;
    }
}

fn main() -> std::io::Result<()> {
    let ro_program: Vec<u32> = include_str!("../input")
        .split(',')
        .map(|s| s.parse::<u32>().expect("Not an unsigned integer"))
        .collect();

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let result = execute_program(&ro_program, noun, verb);

            if result == 19_690_720 {
                println!("prog({:?}, {:?}) = {:?}", noun, verb, result);

                let answer = 100 * noun + verb;
                println!("Answer: {:?}", answer);
                break 'outer;
            }
        }
    }

    Ok(())
}
