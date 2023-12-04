fn main() {
    let content_file = std::fs::read_to_string("./input.txt").unwrap();
    let mut sum: u32 = 0;
    for line in content_file.lines() {
        let sets = get_sets(line);
        let mut iter_set = sets.iter().map(|s| Set::new(s));
        let first = iter_set.next().expect("ERROR: can't get first iter item");

        let mut final_set: Set = Set {
            red: first.red,
            green: first.green,
            blue: first.blue,
        };
        for s in iter_set {
            if s.red > final_set.red {
                final_set.red = s.red
            };
            if s.green > final_set.green {
                final_set.green = s.green
            };
            if s.blue > final_set.blue {
                final_set.blue = s.blue
            };
        }
        sum += final_set.get_power();
    }

    println!("Power sum of sets : {sum}");
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
        if gt {
            return Some(std::cmp::Ordering::Greater);
        };
        let lt = self.blue < other.blue || self.red < other.red || self.green < other.green;
        if lt {
            return Some(std::cmp::Ordering::Less);
        };
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

    pub fn get_power(&self) -> u32 {
        let mut res = 1;
        if self.red != 0 {
            res *= self.red
        };
        if self.green != 0 {
            res *= self.green
        };
        if self.blue != 0 {
            res *= self.blue
        };
        res
    }
}
