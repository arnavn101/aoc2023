use std::collections::HashSet;

fn parse_lines(
    lines: &Vec<String>,
) -> (
    usize,
    usize,
    HashSet<(usize, usize)>,
    HashSet<(usize, usize)>,
) {
    let (mut rounded, mut cube) = (HashSet::new(), HashSet::new());
    let (m, n) = (lines.len(), lines[0].len());

    for (i, line) in lines.iter().rev().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'O' => rounded.insert((i + 1, j + 1)),
                '#' => cube.insert((i + 1, j + 1)),
                '.' => true,
                _ => panic!("Invalid input"),
            };
        }
    }

    (m, n, rounded, cube)
}

fn move_rocks_to_dir(
    m: usize,
    n: usize,
    rounded: &HashSet<(usize, usize)>,
    cube: &HashSet<(usize, usize)>,
    dir: (i32, i32),
) -> HashSet<(usize, usize)> {
    let mut new_rounded = HashSet::new();
    let (dx, dy) = dir;

    let mut sorted_rounded = rounded.iter().collect::<Vec<_>>();
    sorted_rounded.sort_by(|a, b| {
        if dx > 0 {
            b.0.cmp(&a.0)
        } else if dx < 0 {
            a.0.cmp(&b.0)
        } else if dy > 0 {
            b.1.cmp(&a.1)
        } else {
            a.1.cmp(&b.1)
        }
    });

    for &rock in sorted_rounded.iter() {
        let (mut r, mut c) = rock.clone();

        while (dx > 0 && r < m) || (dx < 0 && r > 1) || (dy > 0 && c < n) || (dy < 0 && c > 1) {
            let (next_r, next_c) = ((r as i32 + dx) as usize, (c as i32 + dy) as usize);
            if cube.contains(&(next_r, next_c)) || new_rounded.contains(&(next_r, next_c)) {
                break;
            }
            (r, c) = (next_r, next_c);
        }

        new_rounded.insert((r, c));
    }

    new_rounded
}

fn rounded_to_string(rounded: &HashSet<(usize, usize)>) -> String {
    let mut sorted_rounded = rounded.iter().collect::<Vec<_>>();
    sorted_rounded.sort_by(|a, b| a.cmp(b));
    sorted_rounded
        .iter()
        .map(|(r, c)| format!("{},{}", r, c))
        .collect::<Vec<_>>()
        .join("|")
}

fn string_to_rounded(s: &str) -> HashSet<(usize, usize)> {
    s.split("|")
        .map(|x| {
            let mut iter = x.split(",");
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<HashSet<_>>()
}

fn move_cycles(
    m: usize,
    n: usize,
    rounded: &HashSet<(usize, usize)>,
    cube: &HashSet<(usize, usize)>,
    num_cycles: usize,
) -> HashSet<(usize, usize)> {
    let mut new_rounded = rounded.clone();
    let mut rounded_str = rounded_to_string(&new_rounded);

    // Keep track of previous rounded strings
    let mut seen_rounded: HashSet<String> = HashSet::from([rounded_str.clone()]);
    let mut list_rounded: Vec<String> = vec![rounded_str.clone()];

    for cycle in 0..num_cycles {
        // Go through all directions
        for &dir in [(1, 0), (0, -1), (-1, 0), (0, 1)].iter() {
            new_rounded = move_rocks_to_dir(m, n, &new_rounded, cube, dir);
        }

        // Convert into hashable string
        rounded_str = rounded_to_string(&new_rounded);

        if seen_rounded.contains(&rounded_str) {
            // Find start of the cycle (index of rounded_str)
            let cycle_start = list_rounded.iter().position(|x| *x == rounded_str).unwrap();

            // Final rounded string is at idx where the cycle starts + the remainder of the cycle
            let idx = (num_cycles - cycle_start) % (cycle + 1 - cycle_start) + cycle_start;

            // Convert back to rounded and return
            let final_rounded_str = list_rounded.get(idx).unwrap();
            return string_to_rounded(final_rounded_str);
        }

        seen_rounded.insert(rounded_str.clone());
        list_rounded.push(rounded_str.clone());
    }

    new_rounded
}

fn calc_load(moved_rounded: &HashSet<(usize, usize)>) -> usize {
    moved_rounded.iter().map(|(r, _)| r).sum()
}

pub fn p1(lines: &Vec<String>) -> usize {
    let (m, n, rounded, cube) = parse_lines(lines);
    let moved_rounded = move_rocks_to_dir(m, n, &rounded, &cube, (1, 0));
    calc_load(&moved_rounded)
}

pub fn p2(lines: &Vec<String>) -> usize {
    let (m, n, rounded, cube) = parse_lines(lines);
    let moved_rounded: HashSet<(usize, usize)> = move_cycles(m, n, &rounded, &cube, 1000000000);
    calc_load(&moved_rounded)
}
