use std::collections::HashMap;

use crate::input::*;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Clone)]
enum PartType {
    PartNumber,
    Character,
    Something,
}

#[derive(Clone)]
struct EnginePart {
    begin: Point,
    end: Point,
    part_type: PartType,
    part: String,
}

pub fn day3(session_id: &String) {
    println!("Day 3");
    let input = lines_from_day(3, session_id);
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: Vec<String>) {
    println!("Part 1");

    let test_input = vec![
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string(),
    ];

    let locations = setup_locations(test_input);
    /*locations.values().for_each(|engine| {
        println!(
            "part: {} at {},{}",
            engine.part, engine.begin.x, engine.begin.y
        )
    });*/
    let output: i32 = locations
        .values()
        .filter(|part| is_part_number(part) && is_near_character(part, &locations))
        .map(|part| {
            // println!("part with character: {}", part.part);
            part.part.parse::<i32>().unwrap()
        })
        .sum();
    println!("Test: {}", output);

    let locations = setup_locations(input);
    /*locations
    .values()
    .filter(|engine| !is_part_number(&engine))
    .for_each(|engine| {
        println!(
            "part: {} at {},{}",
            engine.part, engine.begin.x, engine.begin.y
        )
    });*/
    /*let simple_part = &EnginePart {
        part_type: PartType::Something,
        begin: Point { x: 0, y: 0 },
        end: Point { x: 0, y: 0 },
        part: String::from("."),
    };

    let mut skip_amount = 0;
    for y in 0..140 {
        let mut line = String::from("");
        for x in 0..140 {
            if skip_amount > 0 {
                skip_amount -= 1;
                continue;
            }
            let item = locations
                .get(&Point { x, y })
                .unwrap_or(simple_part)
                .part
                .as_ref();
            line += item;
            if item.len() > 1 {
                skip_amount = item.len() as i32 - 1;
            }
        }
        println!("{line}")
    }*/
    let output: i32 = locations
        .values()
        .filter(|part| is_part_number(part) && is_near_character(part, &locations))
        .map(|part| {
            //println!("part with character: {}", part.part);
            part.part.parse::<i32>().unwrap()
        })
        .sum();
    println!("Output: {}", output);
}

fn is_part_number(part: &EnginePart) -> bool {
    part.part_type == PartType::PartNumber
}

fn is_near_character(part: &EnginePart, locations: &HashMap<Point, EnginePart>) -> bool {
    if !is_part_number(part) {
        return false;
    }
    /*println!(
        "checking part: {} at {},{}",
        part.part, part.begin.x, part.begin.y
    );*/
    for x in part.begin.x..part.end.x + 1 {
        if connected_to_character(Point { x, y: part.begin.y }, locations) {
            //println!("connected part: {} at {},{}", part.part, x, part.begin.y);
            return true;
        }
    }
    false
}

fn connected_to_character(point: Point, locations: &HashMap<Point, EnginePart>) -> bool {
    locations
        .get(&Point {
            x: point.x - 1,
            y: point.y,
        })
        .unwrap_or(&EnginePart {
            part_type: PartType::Something,
            begin: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 0 },
            part: String::from("Something"),
        })
        .part_type
        == PartType::Character
        || locations
            .get(&Point {
                x: point.x - 1,
                y: point.y - 1,
            })
            .unwrap_or(&EnginePart {
                part_type: PartType::Something,
                begin: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 0 },
                part: String::from("Something"),
            })
            .part_type
            == PartType::Character
        || locations
            .get(&Point {
                x: point.x,
                y: point.y - 1,
            })
            .unwrap_or(&EnginePart {
                part_type: PartType::Something,
                begin: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 0 },
                part: String::from("Something"),
            })
            .part_type
            == PartType::Character
        || locations
            .get(&Point {
                x: point.x + 1,
                y: point.y - 1,
            })
            .unwrap_or(&EnginePart {
                part_type: PartType::Something,
                begin: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 0 },
                part: String::from("Something"),
            })
            .part_type
            == PartType::Character
        || locations
            .get(&Point {
                x: point.x + 1,
                y: point.y,
            })
            .unwrap_or(&EnginePart {
                part_type: PartType::Something,
                begin: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 0 },
                part: String::from("Something"),
            })
            .part_type
            == PartType::Character
        || locations
            .get(&Point {
                x: point.x + 1,
                y: point.y + 1,
            })
            .unwrap_or(&EnginePart {
                part_type: PartType::Something,
                begin: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 0 },
                part: String::from("Something"),
            })
            .part_type
            == PartType::Character
        || locations
            .get(&Point {
                x: point.x,
                y: point.y + 1,
            })
            .unwrap_or(&EnginePart {
                part_type: PartType::Something,
                begin: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 0 },
                part: String::from("Something"),
            })
            .part_type
            == PartType::Character
        || locations
            .get(&Point {
                x: point.x - 1,
                y: point.y + 1,
            })
            .unwrap_or(&EnginePart {
                part_type: PartType::Something,
                begin: Point { x: 0, y: 0 },
                end: Point { x: 0, y: 0 },
                part: String::from("Something"),
            })
            .part_type
            == PartType::Character
}

fn setup_locations(input: Vec<String>) -> HashMap<Point, EnginePart> {
    let mut locations = HashMap::new();

    for (y, line) in input.into_iter().enumerate() {
        let mut x = 0;
        let mut part = String::from("");
        let mut part_type = PartType::PartNumber;
        let mut item_parsing = false;
        let mut starting_location = Point { x: 0, y: 0 };
        line.chars().for_each(|item| {
            if item.is_numeric() {
                part += &String::from(item);
                part_type = PartType::PartNumber;
                if !item_parsing {
                    starting_location.x = x;
                    starting_location.y = y as i32;
                }
                item_parsing = true
            } else {
                match item {
                    '.' => {
                        if item_parsing {
                            locations.insert(
                                starting_location.clone(),
                                EnginePart {
                                    begin: starting_location.clone(),
                                    end: Point {
                                        x: x - 1,
                                        y: y as i32,
                                    },
                                    part_type: part_type.clone(),
                                    part: part.clone(),
                                },
                            );
                            item_parsing = false;
                            part = String::from("");
                        }
                    }
                    _ => {
                        if item_parsing {
                            locations.insert(
                                starting_location.clone(),
                                EnginePart {
                                    begin: starting_location.clone(),
                                    end: Point {
                                        x: x - 1,
                                        y: y as i32,
                                    },
                                    part_type: part_type.clone(),
                                    part: part.clone(),
                                },
                            );
                            item_parsing = false;
                            part = String::from("");
                        }

                        if !item_parsing {
                            locations.insert(
                                Point { x, y: y as i32 },
                                EnginePart {
                                    begin: Point { x, y: y as i32 },
                                    end: Point { x, y: y as i32 },
                                    part_type: PartType::Character,
                                    part: String::from(item),
                                },
                            );
                        }
                    }
                };
            }
            x += 1;
        });
        if item_parsing {
            locations.insert(
                starting_location.clone(),
                EnginePart {
                    begin: starting_location.clone(),
                    end: Point {
                        x: x - 1,
                        y: y as i32,
                    },
                    part_type: part_type.clone(),
                    part: part.clone(),
                },
            );
        }
    }

    locations
}

fn part2(input: Vec<String>) {
    println!("Part 2");
    let test_input = vec![
        "467..114..".to_string(),
        "...*......".to_string(),
        "..35..633.".to_string(),
        "......#...".to_string(),
        "617*......".to_string(),
        ".....+.58.".to_string(),
        "..592.....".to_string(),
        "......755.".to_string(),
        "...$.*....".to_string(),
        ".664.598..".to_string(),
    ];
    let locations = setup_locations(test_input);
    let gear_locations: Vec<_> = locations
        .values()
        .filter(|engine_part| engine_part.part.contains('*'))
        .collect();

    let mut output = 0;
    gear_locations.iter().for_each(|gear| {
        let gear_parts: Vec<_> = locations
            .values()
            .filter(|part| is_part_number(part) && is_near_current_gear(part, gear))
            .map(|part| part.part.parse::<i32>().unwrap())
            .collect();
        if gear_parts.len() == 2 {
            //println!("part near gear: {:?}", gear_parts);
            let mut gear_ratio = 1;
            gear_parts
                .iter()
                .for_each(|gear_part| gear_ratio *= gear_part);
            output += gear_ratio;
        }
    });

    println!("Test: {}", output);

    let locations = setup_locations(input);

    let gear_locations: Vec<_> = locations
        .values()
        .filter(|engine_part| engine_part.part == "*")
        .collect();

    let mut output = 0;
    gear_locations.iter().for_each(|gear| {
        let gear_parts: Vec<_> = locations
            .values()
            .filter(|part| is_part_number(part) && is_near_current_gear(part, gear))
            .map(|part| {
                //println!("part with character: {}", part.part);
                part.part.parse::<i32>().unwrap()
            })
            .collect();
        if gear_parts.len() == 2 {
            //println!("part near gear: {:?}", gear_parts);
            let mut gear_ratio = 1;
            gear_parts
                .iter()
                .for_each(|gear_part| gear_ratio *= gear_part);
            output += gear_ratio;
        }
    });
    println!("Output: {}", output);
}

fn is_near_current_gear(part: &EnginePart, gear: &EnginePart) -> bool {
    if !is_part_number(part) {
        return false;
    }
    for x in part.begin.x..part.end.x + 1 {
        if connected_to_gear(Point { x, y: part.begin.y }, gear) {
            return true;
        }
    }
    false
}

fn connected_to_gear(location: Point, gear: &EnginePart) -> bool {
    !(location.x - 1 != gear.begin.x
        && location.x != gear.begin.x
        && location.x + 1 != gear.begin.x
        || location.y != gear.begin.y
            && location.y - 1 != gear.begin.y
            && location.y + 1 != gear.begin.y
        || location.x - 1 != gear.begin.x
            && location.y - 1 != gear.begin.y
            && location.x + 1 != gear.begin.x
            && location.y + 1 != gear.begin.y)
}
