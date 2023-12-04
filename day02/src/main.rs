use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // open input file
    let file = File::open("./input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // set maximum values for each color
    let max_red: i32 = 12;
    let max_green: i32 = 13;
    let max_blue: i32 = 14;

    // the approach will be to map each line to either 0 or its games id, the answer will be the sum of the mapped lines
    // map line by line
    let lines: Vec<i32> = reader
        .lines()
        .map(|line| line.expect("line read failure"))
        .map(|line| {
            // split the line into game_id and game
            let game_id: &str = line.split(":").collect::<Vec<&str>>()[0];
            let game_id_value: i32 = String::from_iter(game_id.chars().filter(|c| c.is_numeric()))
                .parse()
                .expect("parse error");
            let game: &str = line.split(":").collect::<Vec<&str>>()[1];

            // split the game into rounds
            let rounds: Vec<&str> = game.split(";").collect();
            for round in rounds {
                
                // split the round into color counts
                let counts: Vec<&str> = round.split(",").collect();
                for count in counts {
                    
                    // split each count into color and value
                    let color: String = count.chars().filter(|c| !c.is_numeric()).collect();
                    let value: i32 = String::from_iter(count.chars().filter(|c| c.is_numeric()))
                        .parse()
                        .expect("parse error");

                    // check if the value is greater than the max for the color
                    // if so, return 0 to be summed
                    if color.contains("red") && value > max_red {
                        return 0;
                    } else if color.contains("green") && value > max_green {
                        return 0;
                    } else if color.contains("blue") && value > max_blue {
                        return 0;
                    }
                }
            }
            // return the game_id if all the values are valid
            return game_id_value;
        })
        .collect();

    // sum the lines
    let sum: i32 = lines.iter().sum();
    //print the sum
    println!("Sum: {}", sum);
}
