use reqwest::header::COOKIE;

pub fn lines_from_day(day: u32, session_id: &String) -> Vec<String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/2023/day/{day}/input");
    let resp = match client
        .get(url)
        .header(COOKIE, format!("session={session_id}"))
        .send()
    {
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
