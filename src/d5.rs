/*

*/

use std::collections::HashMap;

fn get_mapping() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("seed-to-soil", "soil-to-fertilizer"),
        ("soil-to-fertilizer", "fertilizer-to-water"),
        ("fertilizer-to-water", "water-to-light"),
        ("water-to-light", "light-to-temperature"),
        ("light-to-temperature", "temperature-to-humidity"),
        ("temperature-to-humidity", "humidity-to-location"),
        ("humidity-to-location", "end"),
    ])
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_lines(lines: &Vec<String>) -> (Vec<i64>, HashMap<&str, Vec<(i64, i64, i64)>>) {
    let mut ht: HashMap<&str, Vec<(i64, i64, i64)>> = HashMap::new();

    let seeds: Vec<i64> = parse_line(lines.get(0).unwrap().split(':').nth(1).unwrap().trim());

    let mut cur_key: &str = "";

    lines.iter().skip(1).for_each(|line| {
        if line.is_empty() {
            return;
        }

        if line.contains(':') {
            let mut split = line.split(":");
            cur_key = split
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .next()
                .unwrap();
        } else {
            if !ht.contains_key(cur_key) {
                ht.insert(cur_key, Vec::new());
            }

            let v = parse_line(&line);
            ht.get_mut(cur_key).unwrap().push((v[0], v[1], v[2]));
        }
    });

    (seeds, ht)
}

pub fn p1(lines: &Vec<String>) -> i64 {
    let (seeds, ht) = parse_lines(lines);
    let mapping = get_mapping();

    let mut cur_key = "seed-to-soil";
    let mut cur_vals: Vec<i64> = seeds.clone();
    let mut cur_mod: Vec<bool> = vec![false; cur_vals.len()];

    while cur_key != "end" {
        for &(start_dest, start_src, length) in ht.get(cur_key).unwrap() {
            for i in 0..cur_vals.len() {
                if !cur_mod[i] {
                    let cv = *cur_vals.get(i).unwrap();
                    if cv >= start_src && cv < start_src + length {
                        cur_vals[i] = start_dest + (cv - start_src);
                        cur_mod[i] = true;
                    }
                }
            }
        }

        cur_mod = cur_mod.iter().map(|_| false).collect::<Vec<bool>>();
        cur_key = mapping.get(cur_key).unwrap();
    }

    cur_vals.iter().min().unwrap().clone()
}

fn is_in_between(start: i64, end: i64, val: i64) -> bool {
    val >= start && val <= end
}

pub fn p2(lines: &Vec<String>) -> i64 {
    let (seeds, ht) = parse_lines(lines);
    let mapping = get_mapping();

    let mut cur_key = "seed-to-soil";
    let mut cur_ranges: Vec<(i64, i64, bool)> = Vec::new();

    for i in (0..seeds.len() - 1).step_by(2) {
        cur_ranges.push((seeds[i], seeds[i] + seeds[i + 1] - 1, false));
    }

    while cur_key != "end" {
        for &(start_dest, start_src, length) in ht.get(cur_key).unwrap() {
            for i in 0..cur_ranges.len() {
                let (cur_start, cur_end, cur_mod) = *cur_ranges.get_mut(i).unwrap();
                let end_src = start_src + length - 1;
                let end_dst = start_dest + length - 1;

                let map_offset = start_dest - start_src;

                let map_start = cur_start + map_offset;
                let map_end = cur_end + map_offset;

                if !cur_mod {
                    if is_in_between(start_src, end_src, cur_start)
                        && is_in_between(start_src, end_src, cur_end)
                    {
                        cur_ranges[i].0 = map_start;
                        cur_ranges[i].1 = map_end;
                        cur_ranges[i].2 = true;
                    } else if is_in_between(start_src, end_src, cur_start) {
                        cur_ranges[i].0 = end_src + 1;
                        cur_ranges.push((map_start, end_dst, true));
                    } else if is_in_between(start_src, end_src, cur_end) {
                        cur_ranges[i].1 = start_src - 1;
                        cur_ranges.push((start_dest, map_end, true));
                    }
                }
            }
        }

        for i in 0..cur_ranges.len() {
            cur_ranges[i].2 = false;
        }
        cur_key = mapping.get(cur_key).unwrap();
    }

    cur_ranges
        .iter()
        .map(|&(start, _, _)| start)
        .min()
        .unwrap()
        .clone()
}
