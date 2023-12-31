// Point are represented as :
// [x, y]
//
// Map is represented as :
// [
//  [".", ".", "0"],
//  [...],
// ]
// the "0" is at x=2 and y=0
//
const CARDINAL: [[isize; 2]; 8] = [
    [-1, -1], // top left
    [0, -1],  // top
    [1, -1],  // top right
    [-1, 0],  // left
    [1, 0],   // right
    [-1, 1],  // bottom left
    [0, 1],   // bottom
    [1, 1],   // bottom right
];

#[derive(Debug)]
struct Point {
    pub value: String,
    pub near_symbol: bool,
}

fn main() {
    let file_content = std::fs::read_to_string("./input.txt").unwrap();
    let map: Vec<Vec<&str>> = file_content
        .lines()
        .map(|e| {
            let mut temp = e.split("").collect::<Vec<&str>>();
            temp.remove(0);
            temp.remove(temp.len() - 1);
            // trim start and end
            temp
        })
        .collect::<Vec<Vec<&str>>>();

    let mut final_result: u32 = 0;
    for (pos_y, val_y) in map.iter().enumerate() {
        let mut vec_num: Vec<Point> = Vec::new();
        for (pos_x, val_x) in val_y.iter().enumerate() {
            if val_x.as_bytes().get(0).unwrap().is_ascii_digit() {
                vec_num.push(Point {
                    value: val_x.to_string(),
                    near_symbol: is_near_symbol(&map, [pos_x, pos_y]),
                });
                if pos_x == val_y.len() - 1 {
                    check_num_validity(&mut vec_num, &mut final_result);
                }
            } else {
                if vec_num.len() == 0 {
                    // no number on this line
                    continue;
                }
                check_num_validity(&mut vec_num, &mut final_result);
            }
        }
    }
    println!("final result : {final_result}");
}

fn check_num_validity(vec_num: &mut Vec<Point>, final_result: &mut u32) {
    if vec_num.iter().any(|p| p.near_symbol) {
        let num: u32 = vec_num
            .iter()
            .map(|p| p.value.clone())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap();
        *final_result += num;
    }
    *vec_num = Vec::new();
}

fn is_near_symbol(map: &Vec<Vec<&str>>, point: [usize; 2]) -> bool {
    for cardinal_point in CARDINAL {
        let near_pos_x: isize = (*point.get(0).unwrap() as isize) + cardinal_point.get(0).unwrap();
        let near_pos_y: isize = (*point.get(1).unwrap() as isize) + cardinal_point.get(1).unwrap();
        if near_pos_y < 0
            || near_pos_y > (map.len() as isize) - 1
            || near_pos_x < 0
            || near_pos_x > (map.get(near_pos_y as usize).unwrap().len() as isize) - 1
        {
            // if outside of boundaries, skip this cardinal_point
            continue;
        }

        match map
            .get(near_pos_y as usize)
            .unwrap()
            .get(near_pos_x as usize)
            .unwrap()
            .as_bytes()
            .get(0)
            .unwrap()
        {
            b'.' => continue,
            value => {
                //println!("{}", *value as char);
                if (*value as char).is_digit(10) {
                    continue;
                }
                return true;
            }
        }
    }
    false
}
