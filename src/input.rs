use reqwest::header::COOKIE;

pub fn lines_from_day(day: u32) -> Vec<String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2023/day/{day}/input");
    let resp = match client.get(url).header(COOKIE, "session=53616c7465645f5fba90c988520a74caaa96aa1c29e38e7f232fb703f9c85a2a829749700f4d2a7001fb3975f3a75a58a2eb1c68aabd4c66abfc8c3557f711c3").send() {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err),
    };
    resp.lines().map(|x| x.to_string()).collect::<Vec<String>>()
}

/*pub fn lines_from_file(day: u32) -> Vec<String> {
    let file_path = format!("../input/day-{day}.txt");
    let file = File::open(file_path).expect("No file found for this day: {day}");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line."))
        .collect()
}*/
