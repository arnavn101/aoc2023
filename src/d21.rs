use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Position {
    Plot,
    Rock,
}

fn parse_lines(lines: &Vec<String>) -> (Vec<Vec<Position>>, (i64, i64)) {
    let mut map = Vec::new();
    let mut start = (0, 0);

    for (i, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (j, c) in line.chars().enumerate() {
            match c {
                '.' => row.push(Position::Plot),
                '#' => row.push(Position::Rock),
                'S' => {
                    start = (i as i64, j as i64);
                    row.push(Position::Plot);
                }
                _ => panic!("Unknown character {}", c),
            }
        }
        map.push(row);
    }

    (map, start)
}

fn explore_map(map: &Vec<Vec<Position>>, start: (i64, i64), num_steps: i64) -> usize {
    let mut vis: HashSet<(i64, i64)> = HashSet::new();

    let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let (m, n) = (map.len() as i64, map[0].len() as i64);

    vis.insert(start);

    for _ in 0..num_steps {
        let mut nxt_vis: HashSet<(i64, i64)> = HashSet::new();

        for &(i, j) in vis.iter() {
            for (di, dj) in dirs.iter() {
                let (ddi, ddj) = (i + di, j + dj);
                let (ni, nj) = (ddi.rem_euclid(m) as usize, ddj.rem_euclid(n) as usize);

                if map[ni][nj] == Position::Rock {
                    continue;
                }

                nxt_vis.insert((ddi, ddj));
            }
        }

        vis = nxt_vis;
    }

    vis.len()
}

pub fn p1(lines: &Vec<String>) -> usize {
    let (map, start) = parse_lines(lines);
    explore_map(&map, start, 64)
}

pub fn p2(lines: &Vec<String>) -> usize {
    let (map, start) = parse_lines(lines);
    explore_map(&map, start, 100)
}

// https://www.youtube.com/watch?v=C2dmxCGGH1s&feature=youtu.be
// https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
