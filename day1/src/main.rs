/*
    Day 1
*/

use std::{fs::File, io::{BufRead, BufReader}, num::ParseIntError, str::{FromStr}};


// ============================ Parse Error ============================
#[derive(Debug)]
struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError
    }
}


// ============================ Sum ============================
#[derive(Debug)]
struct Sum {
    first: Option<u32>,
    last: Option<u32>
}


impl FromStr for Sum {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        // Get first and last position of digits
        let mut first_pos: usize = 0; 
        let mut last_pos: usize = string.len() - 1; 
        let mut first: Option<u32> = Default::default(); 
        let mut last: Option<u32> = Default::default();
        let string_chars: Vec<_> = string.chars().collect();
        for _i in 0..string.len() {
            let first_char = string_chars[first_pos];
            let last_char = string_chars[last_pos];
            if first_char.is_numeric() && !first.is_some() {
                first = first_char.to_digit(10);
            } else if !first.is_some() {
                first_pos += 1;
            }
            if last_char.is_numeric() {
                last = last_char.to_digit(10);
            } else if !last.is_some() {
                last_pos -= 1;
            }
            if first.is_some() && last.is_some() {
                break;
            }
        }

        // Get first and last position of words
        let words = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

        for (i, word) in words.iter().enumerate() {
            if !string.contains(word) {
                continue;
            }
            let mut pos = string.find(word).unwrap();
            if pos < first_pos {
                first_pos = pos;
                first = Some((i + 1) as u32);
            }
            pos = string.rfind(word).unwrap();
            if pos > last_pos {
                last_pos = pos;
                last = Some((i + 1) as u32);
            }
        }

        Ok(Sum {
            first: first,
            last: last
        })
    }
}




// ============================ Main ============================
fn main() {
    // Parsing the lines of instructions
    println!("Parsing lines...");
    let file = File::open("resources/input").unwrap();
    let buf_read = BufReader::new(file);
    let lines = buf_read.lines();

    let mut sums = Vec::<Sum>::new();

    for line in lines {
        if let Ok(line) = line {
            sums.push(line.parse().unwrap());
        } else {
            panic!("The line wasn't very parseable...");
        }
    }
    // println!("Sums:\n{:#?}", sums);

    let mut result_value:u32 = 0;
    for sum in sums {
        result_value += sum.first.unwrap() * 10 + sum.last.unwrap();
    }
    println!("Result: {}", result_value);
}
