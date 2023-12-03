/*
    Day 2
*/

use std::{fs::File, io::{BufRead, BufReader}, num::ParseIntError, str::FromStr, vec};

// ============================ Parse Error ============================
#[derive(Debug)]
struct ParseError;
impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> Self {
        ParseError
    }
}

// ============================ Set ============================
#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32
}

// ============================ Game ============================
#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>
}


impl FromStr for Game {
    type Err = ParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut sets: Vec<Set> = vec![];
        let mut line = string.to_string();

        line = line.replace("Game ", "");
        let id = line.get(..line.find(":").unwrap()).unwrap().parse::<u32>().unwrap();
        line = line.get(line.find(" ").unwrap()..).unwrap().to_string();

        let mut done = false;
        while !done {
            let mut color_values = [0, 0, 0];
            let mut end_index = 0;
            if line.contains(";") {
                end_index = line.find(";").unwrap() + 1;
            } else {
                end_index = line.len();
                done = true;
            }
            let mut set_string = line.get(..end_index).unwrap().to_string();
            line = line.replace(&set_string, "");
            set_string = set_string.replace(";", "");

            let words = ["red", "green", "blue"];
            for (i, word) in words.iter().enumerate() {
                if set_string.contains(word) {
                    let mut color_string = set_string.get(..set_string.find(word).unwrap() - 1).unwrap().to_string();
                    color_string = color_string.get(color_string.rfind(" ").unwrap()..).unwrap().to_string().replace(" ", "");
                    color_values[i] = color_string.parse::<u32>().unwrap();
                }
            }

            sets.push(Set{
                red: color_values[0],
                green: color_values[1],
                blue: color_values[2]
            });
        }


        Ok(Game {
            id: id,
            sets: sets
        })
    }
}




// ============================ Main ============================
fn main() {
    // Parsing the lines of instructions
    println!("Parsing lines...");
    let file = File::open("day2/resources/input").unwrap();
    let buf_read = BufReader::new(file);
    let lines = buf_read.lines();

    let mut games = Vec::<Game>::new();

    for line in lines {
        if let Ok(line) = line {
            games.push(line.parse().unwrap());
        } else {
            panic!("The line wasn't very parseable...");
        }
    }

    let cubes_red = 12;
    let cubes_green =  13;
    let cubes_blue = 14;

    let mut result_value:u32 = 0;
    for game in &games {
        let mut game_is_possible = true;
        for set in &game.sets {
            game_is_possible &= set.red <= cubes_red && set.green <= cubes_green && set.blue <= cubes_blue;
        }
        if game_is_possible {
            result_value += game.id;
        }
    }
    println!("Result: {}", result_value);

    
    result_value = 0;
    for game in &games {
        let mut minimum_red = 0;
        let mut minimum_green = 0;
        let mut minimum_blue = 0;
        for set in &game.sets {
            if set.red > minimum_red { minimum_red = set.red; }
            if set.green > minimum_green { minimum_green = set.green; }
            if set.blue > minimum_blue { minimum_blue = set.blue; }
        }
        println!("Power: {}", minimum_red * minimum_green * minimum_blue);
        result_value += minimum_red * minimum_green * minimum_blue;
    }
    println!("Result: {}", result_value);   // 78057 is too low
}
