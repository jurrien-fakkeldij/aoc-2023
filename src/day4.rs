use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::input::lines_from_day;

pub fn day4() {
    println!("Day 4");
    let input = lines_from_day(4);
    part1(input.clone());
    part2(input.clone());
}

struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    actual_numbers: HashSet<i32>,
}

fn part1(input: Vec<String>) {
    println!("Part 1");

    let test_input = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
    ];

    let games = setup_games(test_input);

    let output: i32 = games
        .values()
        .map(|card: &Card| {
            let matching = card
                .actual_numbers
                .intersection(&card.winning_numbers)
                .count();
            if matching == 0 {
                return 0;
            }
            i32::pow(2, (matching as u32) - 1)
        })
        .sum();
    println!("Test: {}", output);

    let games = setup_games(input);

    let output: i32 = games
        .values()
        .map(|card: &Card| {
            let matching = card
                .actual_numbers
                .intersection(&card.winning_numbers)
                .count();
            if matching == 0 {
                return 0;
            }
            i32::pow(2, (matching as u32) - 1)
        })
        .sum();
    println!("Output: {}", output);
}

fn part2(input: Vec<String>) {
    println!("Part 2");

    let test_input = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
    ];

    let games = setup_games(test_input);
    let mut copies: HashMap<i32, i32> = HashMap::new();
    games.keys().sorted().for_each(|card_id| {
        let card = games.get(card_id).unwrap();
        let matching = card
            .actual_numbers
            .intersection(&card.winning_numbers)
            .count();

        let amount_copies = *copies.get(&card.id).unwrap_or(&0);

        for card_number in (card_id + 1)..(card_id + 1 + matching as i32) {
            if !copies.contains_key(&card_number) {
                copies.insert(card_number, 1 + amount_copies);
            } else {
                copies
                    .entry(card_number)
                    .and_modify(|i| *i += 1 + amount_copies);
            }
        }
    });
    println!("{:?}", copies);
    let output: i32 = copies.values().sum::<i32>() + games.keys().len() as i32;
    println!("Test: {}", output);

    let games = setup_games(input);
    let mut copies: HashMap<i32, i32> = HashMap::new();
    games.keys().sorted().for_each(|card_id| {
        let card = games.get(card_id).unwrap();
        let matching = card
            .actual_numbers
            .intersection(&card.winning_numbers)
            .count();

        let amount_copies = *copies.get(&card.id).unwrap_or(&0);

        for card_number in (card_id + 1)..(card_id + 1 + matching as i32) {
            if !copies.contains_key(&card_number) {
                copies.insert(card_number, 1 + amount_copies);
            } else {
                copies
                    .entry(card_number)
                    .and_modify(|i| *i += 1 + amount_copies);
            }
        }
    });
    let output: i32 = copies.values().sum::<i32>() + games.keys().len() as i32;
    println!("Output: {}", output);
}

fn setup_games(input: Vec<String>) -> HashMap<i32, Card> {
    let mut games = HashMap::new();
    input.iter().for_each(|line| {
        let mut game_num = 0;
        let mut winning_numbers: HashSet<i32> = HashSet::new();
        let mut actual_numbers: HashSet<i32> = HashSet::new();
        line.split(':').for_each(|part| {
            if part.contains("Card") {
                game_num = part[5..].trim().parse().unwrap();
            } else {
                let number_parts: Vec<String> =
                    part.trim().split('|').map(|s| s.to_string()).collect();
                winning_numbers = number_parts
                    .first()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| -> i32 { x.parse().unwrap() })
                    .collect();

                actual_numbers = number_parts
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .map(|x| -> i32 { x.parse().unwrap() })
                    .collect();
            }
        });

        games.insert(
            game_num,
            Card {
                actual_numbers,
                winning_numbers,
                id: game_num,
            },
        );
    });

    games
}
