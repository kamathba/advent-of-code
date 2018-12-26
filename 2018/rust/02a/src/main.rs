use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut two : u32 = 0;
    let mut three : u32 = 0;
    for line in BufReader::new(file).lines() {
        let mut letters = HashMap::new();
        for letter in line.unwrap().chars() {
            let count = letters.entry(letter).or_insert(0);
            *count += 1;
        }

        if letters.values().find(|&val| *val == 2).is_some() {
            two += 1;
        }

        if letters.values().find(|&val| *val == 3).is_some() {
            three += 1;
        }
    }

    println!("Exactly two: {:?}", two);
    println!("Exactly three: {:?}", three);
    println!("Checksum: {:?}", two*three);

    Ok(())
}
