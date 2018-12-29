#[macro_use]
extern crate nom;

use nom::types::CompleteStr;
use std::fs::File;
use std::io::{BufRead, BufReader};

/**
    Amidst the chaos, you notice that exactly one claim doesn't overlap by even a single square inch of fabric with any other claim. If you can somehow draw attention to it, maybe the Elves will be able to make Santa's suit after all!

    For example, in the claims above, only claim 3 is intact after all claims are made.

    What is the ID of the only claim that doesn't overlap?
*/

#[derive(Debug)]
pub struct Claim {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

fn from_dec(input: CompleteStr) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(&input, 10)
}

fn is_decimal_digit(c: char) -> bool {
    c.is_digit(10)
}

named!(integer<CompleteStr, u32>,
    map_res!(take_while!(is_decimal_digit), from_dec)
);

named!(claim<CompleteStr, Claim>,
    do_parse!(
            tag!("#") >>
        id: integer >>
            tag!(" @ ") >>
        x:  integer >>
            tag!(",") >>
        y:  integer >>
            tag!(": ") >>
        width: integer >>
            tag!("x") >>
        height: integer >>
        (Claim {id: id, x: x, y: y, width: width, height: height})
    )
);

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input")?;
    let mut fabric = [[0u32; 1000]; 1000];
    let mut claims: Vec<Claim> = Vec::new();

    for line in BufReader::new(file).lines() {
        let (_, claim) = claim(CompleteStr(&line.unwrap())).unwrap();
        claims.push(claim);
    }

    for claim in &claims {
        for i in 0..claim.width {
            for j in 0..claim.height {
                let square_inch: &mut u32 =
                    &mut fabric[(claim.x + i) as usize][(claim.y + j) as usize];
                *square_inch += 1;
            }
        }
    }

    'outer: for claim in &claims {
        for i in 0..claim.width {
            for j in 0..claim.height {
                if fabric[(claim.x + i) as usize][(claim.y + j) as usize] != 1 {
                    continue 'outer;
                }
            }
        }

        println!("{:?}", claim);
    }

    Ok(())
}
