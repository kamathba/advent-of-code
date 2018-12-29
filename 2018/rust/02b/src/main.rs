use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut vec = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }

    println!("{} lines in input", vec.len());

    for i in 0..vec.len() {
        'nest: for j in i + 1..vec.len() {
            assert_eq!(vec[i].len(), vec[j].len());
            let mut index: Option<usize> = None;
            let mut chars_i = vec[i].chars();
            let mut chars_j = vec[j].chars();

            let wordlen = vec[i].len();
            for pos in 0..wordlen {
                if chars_i.next() != chars_j.next() {
                    match index.as_mut() {
                        Some(_val) => continue 'nest,
                        None => index = Some(pos),
                    }
                }
            }

            let mut answer = vec[i].clone();
            answer.remove(index.unwrap());

            println!("i: {}, j: {}", i, j);
            println!("i: {}", vec[i]);
            println!("j: {}", vec[j]);
            println!("answer: {}", answer);
        }
    }

    Ok(())
}
