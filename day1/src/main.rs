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
    first: i32,
    last: i32
}


impl FromStr for Sum {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        // Get first and last position of digits
        let mut first_pos: usize = 0; 
        let mut last_pos: usize = string.len() - 1; 
        let mut first: i32 = -1; 
        let mut last: i32 = -1;
        for i in 0..string.len() {
            let string_chars: Vec<_> = string.chars().collect();
            let first_char = string_chars[first_pos];
            let last_char = string_chars[last_pos];
            if first_char.is_numeric() && first == -1 {
                first = first_char.to_digit(10).unwrap() as i32;
            } else if first == -1 {
                first_pos += 1;
            }
            if last_char.is_numeric() && last == -1 {
                last = last_char.to_digit(10).unwrap() as i32;
            } else if last == -1 {
                last_pos -= 1;
            }
            if first > -1 && last > -1 {
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
            if pos >= 0 && pos < first_pos {
                first_pos = pos;
                first = (i + 1) as i32
            }
            pos = string.rfind(word).unwrap();
            if pos > last_pos {
                last_pos = pos;
                last = (i + 1) as i32
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
    println!("Sums:\n{:#?}", sums);

    let mut result_value:i32 = 0;
    for sum in sums {
        result_value += sum.first * 10 + sum.last;
    }
    println!("Result: {}", result_value);
}
