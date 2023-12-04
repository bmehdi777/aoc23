fn main() {
    let content_file = std::fs::read_to_string("./input.txt").unwrap();
    let mut sum: u32 = 0;
    let max_cube: Set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    for line in content_file.lines() {
        let game_id = get_game_id(line);
        let sets = get_sets(line);
        let list_set: Vec<Set> = sets.iter().map(|s| Set::new(s)).collect();
        let mut final_set: Set = Set { red: 0, green: 0, blue: 0};
        for s in list_set {
            if s.red > final_set.red { final_set.red = s.red };
            if s.green > final_set.green { final_set.green = s.green };
            if s.blue > final_set.blue { final_set.blue = s.blue };
        }
        if final_set > max_cube { continue; }
        sum += game_id;
    }

    println!("Game id sum : {sum}");
}

fn get_game_id(content_line: &str) -> u32 {
    let id: &str = content_line
        .split(": ")
        .collect::<Vec<&str>>()
        .get(0)
        .expect("ERROR: the line doesn't contain 'Game x'")
        .split(" ")
        .collect::<Vec<&str>>()
        .get(1)
        .expect("ERROR: the game doesn't contain IDs");
    id.parse().expect("ERROR: can't parse ID")
}

fn get_sets(content_line: &str) -> Vec<&str> {
    let sets: Vec<&str> = content_line
        .split(": ")
        .collect::<Vec<&str>>()
        .get(1)
        .expect("ERROR: No game nor sets in line")
        .split("; ")
        .collect::<Vec<&str>>();
    sets
}


#[derive(Debug, PartialEq, Eq)]
struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl<'a, 'b> PartialOrd<Set> for Set {
    fn partial_cmp(&self, other: &Set) -> Option<std::cmp::Ordering> {
        let gt = self.blue > other.blue || self.red > other.red || self.green > other.green;
        if gt { return Some(std::cmp::Ordering::Greater) };
        let lt = self.blue < other.blue || self.red < other.red || self.green < other.green;
        if lt { return Some(std::cmp::Ordering::Less)};
        Some(std::cmp::Ordering::Equal)
    }
}

impl Set {
    pub fn new(set: &str) -> Self {
        let mut s = Set {
            red: 0,
            green: 0,
            blue: 0,
        };
        let colors_num = set.split(", ").collect::<Vec<&str>>();
        for col_num in colors_num {
            let split = col_num.split(" ").collect::<Vec<&str>>();
            let num: u32 = split
                .get(0)
                .expect("ERROR: no num in this set")
                .parse()
                .expect("ERROR: can't parse the num into u32");
            let col = split.get(1).expect("ERROR: no color in this set");
            match *col {
                "red" => s.red = num,
                "green" => s.green = num,
                "blue" => s.blue = num,
                _ => panic!("ERROR: the color found doesn't exist"),
            }
        }
        s
    }
}
