use std::collections::HashMap;

fn parse_lines(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn get_all_empty_rows_cols(universe: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let mut empty_rows = Vec::new();
    let mut empty_cols = Vec::new();

    for (i, row) in universe.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(i);
        }
    }

    for i in 0..universe[0].len() {
        if universe.iter().all(|row| row[i] == '.') {
            empty_cols.push(i);
        }
    }

    (empty_rows, empty_cols)
}

fn get_all_galaxies(universe: &Vec<Vec<char>>, replace_empty_by: usize) -> Vec<(usize, usize)> {
    let mut galaxies = Vec::new();
    let (empty_rows, empty_cols) = get_all_empty_rows_cols(universe);

    for (i, row) in universe.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                let shifted_i =
                    i + ((replace_empty_by - 1) * empty_rows.iter().filter(|&&x| x < i).count());
                let shifted_j =
                    j + ((replace_empty_by - 1) * empty_cols.iter().filter(|&&x| x < j).count());
                galaxies.push((shifted_i, shifted_j));
            }
        }
    }

    galaxies
}

fn get_all_pairwise_distances(galaxies: &Vec<(usize, usize)>) -> HashMap<(usize, usize), usize> {
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();

    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let galaxy1 = galaxies[i];
            let galaxy2 = galaxies[j];

            let distance = (galaxy1.0).abs_diff(galaxy2.0) + (galaxy1.1).abs_diff(galaxy2.1);
            distances.insert((i + 1, j + 1), distance);
        }
    }

    distances
}

pub fn p1(lines: &Vec<String>) -> usize {
    let universe = parse_lines(lines);
    let galaxies = get_all_galaxies(&universe, 2);
    get_all_pairwise_distances(&galaxies).values().sum()
}

pub fn p2(lines: &Vec<String>) -> usize {
    let universe = parse_lines(lines);
    let galaxies = get_all_galaxies(&universe, 1000000);
    get_all_pairwise_distances(&galaxies).values().sum()
}
