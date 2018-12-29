#[macro_use]
extern crate nom;

use nom::types::CompleteStr;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    let mut overlap: u32 = 0;

    for line in BufReader::new(file).lines() {
        let (_, claim) = claim(CompleteStr(&line.unwrap())).unwrap();

        for i in 0..claim.width {
            for j in 0..claim.height {
                let square_inch: &mut u32 =
                    &mut fabric[(claim.x + i) as usize][(claim.y + j) as usize];
                *square_inch += 1;

                if *square_inch == 2 {
                    overlap += 1;
                }
            }
        }
    }

    println!("Overlapping square inches: {}", overlap);

    Ok(())
}
