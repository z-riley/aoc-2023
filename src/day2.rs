use std::fs;

const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

fn is_valid_round(round: &str) -> bool {
    // Example input: " 7 green, 20 blue, 9 red"
    
    // Split into vectors. E.g., [[7, "green"], [20, "blue"], [9, "red"]]
    let num_colour_pairs: Vec<Vec<&str>> = round
        .split(", ").collect::<Vec<&str>>()
        .iter()
        .map(|&x| x.split(" ")
            .filter(|&s| s != "") // remove blank spaces
            .collect::<Vec<&str>>())
        .collect();

    for num_colour_pair in num_colour_pairs {
        let colour = num_colour_pair[1];
        let num = num_colour_pair[0].parse::<i32>().unwrap();

        if (colour == "red" && num > MAX_RED_CUBES)
            || (colour == "green" && num > MAX_GREEN_CUBES)
            || (colour == "blue" && num > MAX_BLUE_CUBES) {
                return false;
        }
    }

    return true;
}

fn is_valid_game(game: &str) -> bool {

    let results_section = game.split(": ").collect::<Vec<_>>()[1];
    
    let results_row: Vec<&str> = results_section.split(";").collect();
    for round in &results_row {
        if !is_valid_round(round) {
            return false;
        }    
    }

    return true;
}

pub fn main() {

    let data = fs::read_to_string("input/day2.txt").expect("Unable to read file");
    
    let mut sum_of_valid_game_ids = 0;

    for (index, game) in data.lines().enumerate(){

        if is_valid_game(game) {
            let game_id = index + 1;
            sum_of_valid_game_ids += game_id;
        }        
    }

    println!("Sum of valid game IDs: {}", sum_of_valid_game_ids);

}
