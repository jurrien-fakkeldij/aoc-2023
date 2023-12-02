use std::collections::HashMap;

use crate::input::*;

pub fn day2() {
    println!("Day 2");
    let input = lines_from_day(2);
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: Vec<String>) {
    println!("Part 1");

    let test_input = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
    ];
    let games = get_colours_from_games(test_input);
    let output: u32 = games
        .iter()
        .filter(|(_, v)| has_less_or_equal_than_max((12, 13, 14), v))
        .map(|(game_num, _)| game_num)
        .sum();

    println!("Test: {}", output);

    let games = get_colours_from_games(input);

    let output: u32 = games
        .iter()
        .filter(|(_, v)| has_less_or_equal_than_max((12, 13, 14), v))
        .map(|(game_num, _)| game_num)
        .sum();

    println!("Output: {}", output);
}

fn has_less_or_equal_than_max(max: (u32, u32, u32), game: &HashMap<String, u32>) -> bool {
    game.iter()
        .filter(|(k, v)| match k.as_str() {
            "red" => *v <= &max.0,
            "green" => *v <= &max.1,
            "blue" => *v <= &max.2,
            _ => true,
        })
        .collect::<HashMap<_, _>>()
        .len()
        == 3
}

fn get_colours_from_games(input: Vec<String>) -> HashMap<u32, HashMap<String, u32>> {
    let mut games = HashMap::new();

    for line in input {
        let game = parse_game(line);
        games.insert(game.0, game.1);
    }

    games
}

fn parse_game(game: String) -> (u32, HashMap<String, u32>) {
    let mut game_colors = HashMap::new();
    let mut game_num = 0;
    //Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

    game.split(':').for_each(|p| {
        if p.contains("Game") {
            game_num = p[5..].parse().unwrap();
        } else {
            let max_amount_color = calculate_game_numbers(p.trim().to_string());

            game_colors.insert("red".to_string(), max_amount_color.0);
            game_colors.insert("green".to_string(), max_amount_color.1);
            game_colors.insert("blue".to_string(), max_amount_color.2);
        }
    });

    (game_num, game_colors)
}

/* (r,g,b) */
fn calculate_game_numbers(game_results: String) -> (u32, u32, u32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    //println!("game_results:{game_results}");
    game_results.split("; ").for_each(|round| {
        //println!("round:{round}");
        round.split(", ").for_each(|get_cubes| {
            //println!("get_cubes:{get_cubes}");
            let mut amount: u32 = 0;
            get_cubes.split(' ').for_each(|cubes| {
                //println!("cubes:{cubes}");
                match cubes {
                    "red" => {
                        if red < amount {
                            red = amount
                        }
                    }
                    "blue" => {
                        if blue < amount {
                            blue = amount
                        }
                    }
                    "green" => {
                        if green < amount {
                            green = amount
                        }
                    }
                    _ => amount = cubes.parse().unwrap(),
                }
            })
        })
    });

    (red, green, blue)
}

fn part2(input: Vec<String>) {
    println!("Part 2");
    let test_input = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string(),
    ];
    let games = get_colours_from_games(test_input);
    let output: u32 = games
        .values()
        .map(|colors| {
            colors.get("red").unwrap() * colors.get("blue").unwrap() * colors.get("green").unwrap()
        })
        .sum();

    println!("Test: {}", output);

    let games = get_colours_from_games(input);

    let output: u32 = games
        .values()
        .map(|colors| {
            colors.get("red").unwrap() * colors.get("blue").unwrap() * colors.get("green").unwrap()
        })
        .sum();

    println!("Output: {}", output);
}
