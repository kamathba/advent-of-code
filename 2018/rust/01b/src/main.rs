use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let mut freqs = HashSet::new();
    let mut sum: i32 = 0;

    loop {
        let file = File::open("input")?;
        for line in BufReader::new(file).lines() {
            let value = line
                .unwrap()
                .parse::<i32>()
                .expect("Expected lines to be ints");
            sum += value;

            if !freqs.insert(sum) {
                println!("Repeat: {:?}", sum);
                return Ok(());
            }
        }
    }
}
