/**
    --- Day 4: Secure Container ---
    You arrive at the Venus fuel depot only to discover it's protected by a password. The Elves had written the password on a sticky note, but someone threw it out.

    However, they do remember a few key facts about the password:

    It is a six-digit number.
    The value is within the range given in your puzzle input.
    Two adjacent digits are the same (like 22 in 122345).
    Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
    Other than the range rule, the following are true:

    111111 meets these criteria (double 11, never decreases).
    223450 does not meet these criteria (decreasing pair of digits 50).
    123789 does not meet these criteria (no double).
    How many different passwords within the range given in your puzzle input meet these criteria?

    Your puzzle input is 271973-785961.
*/

fn main() -> std::io::Result<()> {
    let in1: u32 = 271_973;
    let in2: u32 = 785_961;

    let mut count: u32 = 0;

    'outer: for i in in1..=in2 {
        let mut digit: u32 = i % 10;
        let mut val: u32 = i;
        let mut double: bool = false;

        for _j in 1..6 {
            val /= 10;
            let next_digit = val % 10;

            if next_digit > digit {
                continue 'outer;
            }

            if next_digit == digit {
                double = true;
            }

            digit = next_digit;
        }

        if !double {
            continue;
        }

        println!("YES: {:?}", i);
        count += 1;
    }

    println!("Count: {:?}", count);

    Ok(())
}
