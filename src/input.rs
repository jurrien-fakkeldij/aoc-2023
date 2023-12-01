use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use reqwest::header::COOKIE;

pub fn lines_from_day(day: u32) -> Vec<String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2023/day/{day}/input");
    let resp = match client.get(url).header(COOKIE, "session=53616c7465645f5fcec928c93be3081bc09f8508a0e383bc7f0927cae5cf60e2b5b524d6f800c4d522e1cdcb019d5fb20712d44a7a26421880af2bbe63ec7533").send() {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    };
    resp.lines().map(|x| x.to_string()).collect::<Vec<String>>()
}

pub fn lines_from_file(day: u32) -> Vec<String> {
    let file_path = format!("../input/day-{day}.txt");
    let file = File::open(file_path).expect("No file found for this day: {day}");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect()
}
