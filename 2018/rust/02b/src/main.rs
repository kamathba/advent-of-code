use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/**
    Confident that your list of box IDs is complete, you're ready to find the boxes full of prototype fabric.

    The boxes will have IDs which differ by exactly one character at the same position in both strings. For example, given the following box IDs:

    abcde
    fghij
    klmno
    pqrst
    fguij
    axcye
    wvxyz
    The IDs abcde and axcye are close, but they differ by two characters (the second and fourth). However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.

    What letters are common between the two correct box IDs? (In the example above, this is found by removing the differing character from either ID, producing fgij.)
*/

fn main() -> Result<()> {
    let file = File::open("input")?;
    let mut vec = Vec::new();
    for line in BufReader::new(file).lines() {
        vec.push(line.unwrap());
    }

    println!("{} lines in input", vec.len());

    /* It is not ideal that the solution to this is a nested loop that permutates all the strings */
    for i in 0..vec.len() {
        'outer: for j in i + 1..vec.len() {
            assert_eq!(vec[i].len(), vec[j].len());
            let mut index: Option<usize> = None;
            let mut chars_i = vec[i].chars();
            let mut chars_j = vec[j].chars();

            let wordlen = vec[i].len();
            for pos in 0..wordlen {
                if chars_i.next() != chars_j.next() {
                    match index.as_mut() {
                        Some(_val) => continue 'outer,
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
