/**
    You've managed to sneak in to the prototype suit manufacturing lab. The Elves are making decent progress, but are still struggling with the suit's size reduction capabilities.

    While the very latest in 1518 alchemical technology might have solved their problem eventually, you can do better. You scan the chemical composition of the suit's material and discover that it is formed by extremely long polymers (one of which is available as your puzzle input).

    The polymer is formed by smaller units which, when triggered, react with each other such that two adjacent units of the same type and opposite polarity are destroyed. Units' types are represented by letters; units' polarity is represented by capitalization. For instance, r and R are units with the same type but opposite polarity, whereas r and s are entirely different types and do not react.

    For example:

    In aA, a and A react, leaving nothing behind.
    In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.
    In abAB, no two adjacent units are of the same type, and so nothing happens.
    In aabAAB, even though aa and AA are of the same type, their polarities match, and so nothing happens.
    Now, consider a larger example, dabAcCaCBAcCcaDA:

    dabAcCaCBAcCcaDA  The first 'cC' is removed.
    dabAaCBAcCcaDA    This creates 'Aa', which is removed.
    dabCBAcCcaDA      Either 'cC' or 'Cc' are removed (the result is the same).
    dabCBAcaDA        No further actions can be taken.
    After all possible reactions, the resulting polymer contains 10 units.

    How many units remain after fully reacting the polymer you scanned?
*/

const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> Result<(), std::io::Error> {
    let file = include_str!("../input");
    let mut min: usize = std::usize::MAX;
    for letter in ALPHABET.chars() {
        let sequence: Vec<char> = file
            .chars()
            .filter(|&x| x != letter && x != letter.to_ascii_lowercase())
            .collect();

        let mut output = String::new();

        let input_len = sequence.len();
        let mut pos: usize = 1;

        output.push(sequence[0] as char);

        loop {
            // We are done processing the input
            if pos >= input_len {
                break;
            }

            // Invariant going into next part of loop is that output is never empty
            if output.is_empty() {
                output.push(sequence[pos]);
                pos += 1;
            }

            let last = output.pop().expect("Output is never empty") as u8;
            let delta = (sequence[pos] as i8) - (last as i8);

            if delta.abs() != 32 {
                output.push(last as char);
                output.push(sequence[pos]);
                pos += 1;
                continue;
            }

            pos += 1;
        }

        let sequence_length = output.len();
        println!("{}: {}", letter, sequence_length);
        if sequence_length < min {
            min = sequence_length;
        }
    }

    println!("Answer: {}", min);
    Ok(())
}
