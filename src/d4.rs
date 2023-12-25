use std::collections::HashSet;

fn split_line(line: &str) -> &str {
    let mut split = line.split(":");
    let snd = split.nth(1).unwrap().trim();
    return snd;
}

fn parse_line(line: &str) -> HashSet<i32> {
    line.split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<HashSet<i32>>()
}

fn parse_lines(lines: &Vec<String>) -> (Vec<HashSet<i32>>, Vec<HashSet<i32>>) {
    let mut v1: Vec<HashSet<i32>> = Vec::new();
    let mut v2: Vec<HashSet<i32>> = Vec::new();

    for line in lines {
        let mut split = line.split("|");
        v1.push(parse_line(split_line(split.next().unwrap())));
        v2.push(parse_line(split.next().unwrap()));
    }

    return (v1, v2);
}

pub fn p1(lines: &Vec<String>) -> usize {
    let (winning, having) = parse_lines(lines);
    let mut tot_pts = 0;

    for (win, have) in winning.iter().zip(having.iter()) {
        let matching = win.intersection(have).count();
        if matching > 0 {
            tot_pts += 2_usize.pow((matching - 1) as u32);
        }
    }

    tot_pts
}

pub fn p2(lines: &Vec<String>) -> usize {
    let (winning, having) = parse_lines(lines);
    let mut tot_cards = 0;
    let mut cur_copies = vec![1; having.len()];

    for (i, (win, have)) in winning.iter().zip(having.iter()).enumerate() {
        let cur_cnt = cur_copies[i];
        tot_cards += cur_cnt;

        let matching = win.intersection(have).count();
        for j in 1..matching + 1 {
            let k = i + j;
            if k < cur_copies.len() {
                cur_copies[k] += cur_cnt;
            }
        }
    }

    tot_cards
}
