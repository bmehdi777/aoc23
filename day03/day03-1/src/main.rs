const CARDINAL: [[isize; 2]; 8] = [
    [-1, -1], // top left
    [0, -1],  // top
    [-1, 1],  // top right
    [-1, 0],  // left
    [1, 0],   // right
    [-1, 1],  // bottom left
    [0, 1],   // bottom
    [1, 1],   // bottom right
];

fn main() {
    let file_content = std::fs::read_to_string("./input.txt").unwrap();
    let map: Vec<Vec<&str>> = file_content
        .lines()
        .map(|e| e.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut num_list: Vec<String> = Vec::new();
    for (y, elm) in map.iter().enumerate() {
        let mut num: String = String::new();
        let has_symbol = false;
        for (x, value) in elm.iter().enumerate() {
            match value.chars().next().unwrap_or(' ') {
                //match value.as_bytes().get(0).unwrap_or(&b' ') {
                val if val.is_digit(10) => {
                    println!("val : {val}");

                    num.push(val);
                }
                _ => {
                    if num.len() > 0 {
                        num_list.push(num.clone());
                        num = String::new();
                    }
                }
            }
        }
    }
    //println!("debug map : {map:?}");
}

fn has_symbol_near(map: &Vec<Vec<&str>>, pos: [usize; 2]) -> bool {
    for card in CARDINAL {
        let near_pos: [isize; 2] = [pos[0] as isize + card[0], pos[1] as isize + card[1]];

        if near_pos[0] < 0 || near_pos[0] > map.get(0).unwrap().len() as isize || 
            near_pos[1] < 0 || near_pos[1] > map.len() as isize {
            continue;
        }

        let item = map.get(near_pos[1] as usize).unwrap().get(near_pos[0] as usize).unwrap().chars().next().unwrap_or(' ') ;

        if   item.is_digit(10) {
            // todo
        }
    }
    false
}
