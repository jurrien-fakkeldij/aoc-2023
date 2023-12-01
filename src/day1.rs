use std::collections::HashMap;

use crate::input::*;

pub fn day1() {
    println!("Day 1");
    let input = lines_from_day(1);
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: Vec<String>) {
    println!("Part 1");

    let test_input = vec![
        "1abc2".to_string(),
        "pqr3stu8vwx".to_string(),
        "a1b2c3d4e5f".to_string(),
        "treb7uchet".to_string(),
    ];

    let output: u32 = test_input
        .iter()
        .map(|l| check_numbers(l.to_string()).parse::<u32>().unwrap())
        .sum();
    // .for_each(|l| println!("{}", check_numbers(l.to_string()).as_str()));

    println!("Test: {}", output);

    let output: u32 = input
        .iter()
        .map(|l| check_numbers(l.to_string()).parse::<u32>().unwrap())
        .sum();

    println!("Output: {}", output);
}

fn check_numbers(input: String) -> String {
    let numbers = input.chars().filter(|a| a.is_numeric()).collect::<Vec<_>>();

    format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap())
}

fn part2(input: Vec<String>) {
    println!("Part 2");
    let test_input = vec![
        "two1nine".to_string(),
        "eightwothree".to_string(),
        "abcone2threexyz".to_string(),
        "xtwone3four".to_string(),
        "4nineeightseven2".to_string(),
        "zoneight234".to_string(),
        "7pqrstsixteen".to_string(),
    ];

    let output: u32 = test_input
        .iter()
        .map(|l| {
            check_numbers_with_written(l.to_string())
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    println!("Test: {}", output);

    let output: u32 = input
        .iter()
        //.for_each(|l| println!("{}", check_numbers(l.to_string()).as_str()));
        .map(|l| {
            check_numbers_with_written(l.to_string())
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    println!("Output: {}", output);
}

fn check_numbers_with_written(input: String) -> String {
    let new_input = check_contains_written(input);
    check_numbers(new_input)
}

fn check_contains_written(input: String) -> String {
    let mut written_list = HashMap::new();
    written_list.insert("one", "1");
    written_list.insert("two", "2");
    written_list.insert("three", "3");
    written_list.insert("four", "4");
    written_list.insert("five", "5");
    written_list.insert("six", "6");
    written_list.insert("seven", "7");
    written_list.insert("eight", "8");
    written_list.insert("nine", "9");
    written_list.insert("zero", "0");

    let mut new_string = String::from("");

    let mut test_string = "".to_string();
    input.chars().for_each(|c| {
        test_string += &c.to_string();
        if written_list.contains_key(test_string.as_str()) {
            new_string += written_list.get(test_string.as_str()).unwrap();
            test_string = String::from(c);
        }

        written_list.iter().for_each(|(k, v)| {
            if test_string.contains(k) {
                new_string += v;
                test_string = String::from(c);
            }
        });
        new_string += &c.to_string();
    });

    new_string
}
