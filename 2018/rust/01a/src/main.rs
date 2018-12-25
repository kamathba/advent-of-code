use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut sum: i32 = 0;
    for line in BufReader::new(file).lines() {
        let value = line
            .unwrap()
            .parse::<i32>()
            .expect("Expected lines to be ints");
        sum += value;
    }

    println!("Frequency: {:?}", sum);

    Ok(())
}
