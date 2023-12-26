use std::collections::{HashMap, HashSet};

fn check_in_range(i: i32, j: i32, m: i32) -> bool {
    i >= 0 && i < m && j >= 0 && j < m
}

fn insert_into_adj(
    adj_list: &mut HashMap<(i32, i32), Vec<(i32, i32)>>,
    (i, j): (i32, i32),
    (k, l): (i32, i32),
    (x, y): (i32, i32),
    m: i32,
) {
    if check_in_range(i, j, m) && check_in_range(k, l, m) {
        adj_list.entry((x, y)).or_insert(Vec::new()).push((i, j));
        adj_list.entry((x, y)).or_insert(Vec::new()).push((k, l));
    }
}

fn parse_lines(lines: &Vec<String>) -> ((i32, i32), HashMap<(i32, i32), Vec<(i32, i32)>>) {
    let mut adj_list = HashMap::new();
    let mut s_idx = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        let m = lines[i].len() as i32;
        for (j, ch) in line.chars().enumerate() {
            let (ii, jj) = (i as i32, j as i32);
            match ch {
                '|' => insert_into_adj(&mut adj_list, (ii + 1, jj), (ii - 1, jj), (ii, jj), m),
                '-' => insert_into_adj(&mut adj_list, (ii, jj + 1), (ii, jj - 1), (ii, jj), m),
                'L' => insert_into_adj(&mut adj_list, (ii - 1, jj), (ii, jj + 1), (ii, jj), m),
                'J' => insert_into_adj(&mut adj_list, (ii - 1, jj), (ii, jj - 1), (ii, jj), m),
                '7' => insert_into_adj(&mut adj_list, (ii + 1, jj), (ii, jj - 1), (ii, jj), m),
                'F' => insert_into_adj(&mut adj_list, (ii + 1, jj), (ii, jj + 1), (ii, jj), m),
                'S' => {
                    // hack - treat S as | to keep the cycle
                    insert_into_adj(&mut adj_list, (ii + 1, jj), (ii - 1, jj), (ii, jj), m);
                    s_idx = (ii, jj);
                }
                '.' => {}
                _ => panic!("Invalid character in input"),
            }
        }
    }

    (s_idx, adj_list)
}

fn process_polygon(
    adj_list: &HashMap<(i32, i32), Vec<(i32, i32)>>,
    s_idx: (i32, i32),
) -> (i32, i32, i32) {
    // Pick's theorem: A = i + b/2 - 1 where A is area, i is number of interior points and b is number of boundary points
    let mut boundary = 0;
    let mut area = 0;

    let mut cur = s_idx.clone();
    let mut vis: HashSet<(i32, i32)> = HashSet::new();

    // do-while hack
    while {
        for &(i, j) in adj_list
            .get(&cur)
            .unwrap_or_else(|| panic!("expected adjacent elements to: {:?}", cur))
        {
            if !vis.contains(&(i, j)) {
                boundary += 1;
                vis.insert((i, j));

                // Shoelace formula where area = 1/2 * (x0*y1 - x1*y0 + ...)
                area += cur.0 * j - cur.1 * i;

                cur = (i, j);
                break;
            }
        }
        cur != s_idx
    } {}

    // Complete Shoelace formula
    area = area / 2;

    // Pick's theorem to find interior points
    let interior_points = area - boundary / 2 + 1;
    (area, interior_points, boundary)
}

pub fn p1(lines: &Vec<String>) -> i32 {
    let (s_idx, adj_list) = parse_lines(lines);
    let boundary = process_polygon(&adj_list, s_idx).2;
    boundary / 2
}

pub fn p2(lines: &Vec<String>) -> i32 {
    let (s_idx, adj_list) = parse_lines(lines);
    process_polygon(&adj_list, s_idx).1
}
