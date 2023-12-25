use std::cmp::Ordering;

fn parse_line(line: &String) -> Vec<i64> {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect()
}

fn concat_line(line: &String) -> i64 {
    line.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string())
        .fold(String::new(), |a, b| a + b.as_str())
        .parse()
        .unwrap()
}

fn parse_lines(lines: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    (parse_line(&lines[0]), parse_line(&lines[1]))
}

fn concat_lines(lines: &Vec<String>) -> (i64, i64) {
    (concat_line(&lines[0]), concat_line(&lines[1]))
}

fn compute_num_ways(time: i64, dist: i64) -> i64 {
    fn compute_dist(hold: i64, time: i64) -> i64 {
        hold * (time - hold)
    }

    let (mut left, mut mid, mut right) = (0, -1, time / 2);

    while left <= right {
        mid = left + (right - left) / 2;
        match compute_dist(mid, time).cmp(&dist) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid - 1,
            _ => break,
        }
    }

    if compute_dist(mid, time) <= dist {
        mid += 1
    }

    let half_num_ways = time / 2 - mid;
    let ways_near_end = if compute_dist(time / 2, time) == compute_dist(time / 2 + 1, time) {
        2
    } else {
        1
    };

    half_num_ways * 2 + ways_near_end
}

pub fn p1(lines: &Vec<String>) -> i64 {
    let (times, distances) = parse_lines(lines);
    let mut num_total_times = 1;

    for (&time, &dist) in times.iter().zip(distances.iter()) {
        num_total_times *= compute_num_ways(time, dist);
    }

    num_total_times
}

pub fn p2(lines: &Vec<String>) -> i64 {
    let (time, dist) = concat_lines(lines);
    compute_num_ways(time, dist)
}
