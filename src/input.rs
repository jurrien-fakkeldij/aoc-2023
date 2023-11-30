use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use reqwest::header::COOKIE;

pub fn lines_from_day(day: u32) -> Vec<String> {
    let client = reqwest::blocking::Client::new();
    let resp = match client.get("https://adventofcode.com/2022/day/1/input").header(COOKIE, "session=53616c7465645f5f29173fc2131a5a331559081fb74d6f83f071fee5477cb6f3f7371a0fed4f00a1e08f572d08bb986989183e0fc103178f3f413940fef04823").send() {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    };
    resp.lines().map(|x| x.to_string()).collect::<Vec<String>>()
}

pub fn lines_from_file(day: u32) -> Vec<String> {
    let file_path = "../input/day-{day}.txt";
    let file = File::open(file_path).expect("No file found for this day: {day}");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect()
}
