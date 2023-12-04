use std::cmp::max;
use std::fs;
fn main() {
    part_two();
}

fn part_two() {
    let file_input = fs::read_to_string("inputs/day_2.txt").expect("Problem reading input file");

    let (MAX_RED, MAX_GREEN, MAX_BLUE) = (12, 13, 14);
    let mut total = 0;
    let mut total_power = 0;

    for (game_number, game_string) in file_input.lines().enumerate() {
        let rounds = game_string.split(":").collect::<Vec<_>>()[1];

        println!("Game number: {game_number}, total so far: {total}");
        let round_strings = rounds.split(";").collect::<Vec<_>>();
        let mut colors: Vec<&str> = Vec::new();
        for round in round_strings {
            colors.extend(round.split(","));
        }

        let mut add_number: bool = false;
        let (mut red_most, mut green_most, mut blue_most) = (0, 0, 0);
        for color in colors {
            let (word, num) = (
                color.split(" ").collect::<Vec<_>>()[2],
                String::from(color.split(" ").collect::<Vec<_>>()[1])
                    .parse()
                    .unwrap(),
            );
            println!("Found number: {num}, color: {word}");
            let max = match word {
                "red" => {
                    red_most = max(red_most, num);
                    MAX_RED
                }
                "blue" => {
                    blue_most = max(blue_most, num);
                    MAX_BLUE
                }
                "green" => {
                    green_most = max(green_most, num);
                    MAX_GREEN
                }
                _ => 10,
            };
            if max >= num {
                add_number = true;
            }
        }

        if add_number {
            total += game_number + 1;
        }
        total_power += red_most * blue_most * green_most;
    }
    println!("Total: {total}");
    println!("Total Power: {total_power}");
}
