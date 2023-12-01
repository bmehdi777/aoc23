use fancy_regex::Regex; // normal regex package doesn't support backtracking
use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|zero|\d))")
        .expect("error with regex");

    let map: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut res = 0;

    for line in input.lines() {
        let capture: Vec<&str> = re
            .captures_iter(line)
            .map(|e| e.unwrap().get(1).unwrap().as_str())
            .collect();
        let mut first = capture.get(0).unwrap();
        let mut last = capture.get(capture.len() - 1).unwrap();
        if let Some(v) = map.get(first) {
            first = v
        }
        if let Some(v) = map.get(last) {
            last = v
        }
        res += format!("{first}{last}").parse::<u32>().unwrap();
    }
    println!("result : {res}");
}
