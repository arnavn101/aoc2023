use std::{cmp, collections::HashMap};

pub fn get_list_cubes(lines: &Vec<String>) -> HashMap<i32, Vec<(i32, i32, i32)>> {
    let mut ht: HashMap<i32, Vec<(i32, i32, i32)>> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let mut vec: Vec<(i32, i32, i32)> = Vec::new();

        let mut game_str = parts.get(1).unwrap().to_string(); //.parse().unwrap();
        game_str.pop();

        let game_id: i32 = game_str.parse().unwrap();

        let mut idx = 2;
        let (mut cur_red, mut cur_green, mut cur_blue) = (0, 0, 0);

        while idx < parts.len() {
            let count: i32 = parts.get(idx).unwrap().parse().unwrap();
            let mut color = parts.get(idx + 1).unwrap().to_string();

            let last_char = color.pop().unwrap();

            if last_char != ';' && last_char != ',' {
                color.push(last_char);
            }

            match color.as_str() {
                "red" => cur_red = count,
                "green" => cur_green = count,
                "blue" => cur_blue = count,
                _ => panic!("Color not recognized"),
            }

            if last_char != ',' {
                vec.push((cur_red, cur_green, cur_blue));
                (cur_red, cur_green, cur_blue) = (0, 0, 0);
            }
            idx += 2;
        }
        ht.insert(game_id, vec);
    }

    ht
}

pub fn p1(all_lines: &Vec<String>) -> i32 {
    let ht = get_list_cubes(all_lines);
    let mut sum_ids = 0;

    ht.iter().for_each(|(&game_id, vec)| {
        if vec.iter().any(|&(r, g, b)| r > 12 || g > 13 || b > 14) {
            sum_ids += game_id;
        }
    });

    sum_ids
}

pub fn p2(all_lines: &Vec<String>) -> i32 {
    let ht = get_list_cubes(all_lines);
    let mut total_pow = 0;

    ht.iter().for_each(|(_, vec)| {
        let (mut mx_r, mut mx_g, mut mx_b) = (0, 0, 0);
        vec.iter().for_each(|&(r, g, b)| {
            mx_r = cmp::max(mx_r, r);
            mx_g = cmp::max(mx_g, g);
            mx_b = cmp::max(mx_b, b);
        });
        total_pow += mx_r * mx_g * mx_b;
    });

    total_pow
}
