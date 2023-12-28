use std::collections::HashSet;

fn parse_lines(lines: &Vec<String>) -> Vec<Vec<String>> {
    let mut all_patterns: Vec<Vec<String>> = Vec::new();
    let mut pattern: Vec<String> = Vec::new();

    lines.iter().for_each(|line| {
        if line.is_empty() {
            all_patterns.push(pattern.clone());
            pattern.clear();
        } else {
            pattern.push(line.to_string());
        }
    });
    all_patterns.push(pattern.clone());

    all_patterns
}

fn find_same_rows_and_cols_within_error(
    pattern: &Vec<String>,
    error: usize,
) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut same_rows: Vec<(usize, usize)> = Vec::new();
    let mut same_cols: Vec<(usize, usize)> = Vec::new();

    let m = pattern.len();
    let n = pattern[0].len();

    for i in 0..m {
        for j in i + 1..m {
            if pattern[i]
                .chars()
                .zip(pattern[j].chars())
                .filter(|(c1, c2)| c1 != c2)
                .count()
                == error
            {
                same_rows.push((i + 1, j + 1));
            }
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            let mut col1 = String::new();
            let mut col2 = String::new();
            for k in 0..m {
                col1.push(pattern[k].chars().nth(i).unwrap());
                col2.push(pattern[k].chars().nth(j).unwrap());
            }
            if col1
                .chars()
                .zip(col2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count()
                == error
            {
                same_cols.push((i + 1, j + 1));
            }
        }
    }

    (same_rows, same_cols)
}

fn find_reflection(
    same_row_or_cols: Vec<(usize, usize)>,
    same_one_off_row_or_cols: Vec<(usize, usize)>,
    last_row_or_col: usize,
) -> (Vec<usize>, Vec<usize>) {
    let rows_or_cols: HashSet<(usize, usize)> = same_row_or_cols.clone().into_iter().collect();
    let one_off_row_or_cols: HashSet<(usize, usize)> =
        same_one_off_row_or_cols.clone().into_iter().collect();

    let mut pos_rows_or_cols: Vec<usize> = Vec::new();
    let mut pos_row_or_cols_with_one_off: Vec<usize> = Vec::new();

    for cur_pos in same_row_or_cols
        .iter()
        .filter(|(i, _)| *i == 1)
        .map(|(i, j)| (*i, *j, false))
        .chain(
            same_row_or_cols
                .iter()
                .filter(|(_, j)| *j == last_row_or_col)
                .map(|(i, j)| (*i, *j, false)),
        )
        .chain(
            same_one_off_row_or_cols
                .iter()
                .filter(|(i, _)| *i == 1)
                .map(|(i, j)| (*i, *j, true)),
        )
        .chain(
            same_one_off_row_or_cols
                .iter()
                .filter(|(_, j)| *j == last_row_or_col)
                .map(|(i, j)| (*i, *j, true)),
        )
    {
        let (mut cur_start, mut cur_end, mut used_one_off) = cur_pos.clone();

        loop {
            if cur_end - cur_start == 1 {
                if used_one_off {
                    pos_row_or_cols_with_one_off.push(cur_start);
                } else {
                    pos_rows_or_cols.push(cur_start);
                }
            }
            cur_start += 1;
            cur_end -= 1;

            if !rows_or_cols.contains(&(cur_start, cur_end)) {
                if !used_one_off && one_off_row_or_cols.contains(&(cur_start, cur_end)) {
                    used_one_off = true;
                    continue;
                } else {
                    break;
                }
            }
        }
    }

    (pos_rows_or_cols, pos_row_or_cols_with_one_off)
}

fn compute_pattern_summary(pattern: &Vec<String>, has_smudge: bool) -> (usize, usize) {
    let (same_rows, same_cols) = find_same_rows_and_cols_within_error(pattern, 0);
    let (mut one_off_rows, mut one_off_cols) = (Vec::new(), Vec::new());

    if has_smudge {
        (one_off_rows, one_off_cols) = find_same_rows_and_cols_within_error(pattern, 1);
    }

    let (reflection_rows, reflection_rows_one_off) =
        find_reflection(same_rows, one_off_rows, pattern.len());
    let (reflection_cols, reflection_cols_one_off) =
        find_reflection(same_cols, one_off_cols, pattern[0].len());

    if (reflection_rows.is_empty() && reflection_cols.is_empty())
        || (has_smudge && reflection_rows_one_off.is_empty() && reflection_cols_one_off.is_empty())
    {
        let str_pattern: String = pattern.join("\n");
        let str_smudge = if has_smudge { " with smudge" } else { "" };
        panic!(
            "No reflection found{} for pattern:\n{}",
            str_smudge, str_pattern
        );
    }

    if has_smudge {
        return (
            reflection_rows_one_off.get(0).unwrap_or(&0).clone(),
            reflection_cols_one_off.get(0).unwrap_or(&0).clone(),
        );
    } else {
        return (
            reflection_rows.get(0).unwrap_or(&0).clone(),
            reflection_cols.get(0).unwrap_or(&0).clone(),
        );
    }
}

fn aggregate_all_patterns(all_patterns: &Vec<Vec<String>>, is_p2: bool) -> usize {
    all_patterns
        .iter()
        .map(|pattern| {
            let (reflection_row, reflection_col) = compute_pattern_summary(pattern, is_p2);
            reflection_col + (100 * reflection_row)
        })
        .sum()
}

pub fn p1(lines: &Vec<String>) -> usize {
    let all_patterns = parse_lines(lines);
    aggregate_all_patterns(&all_patterns, false)
}

pub fn p2(lines: &Vec<String>) -> usize {
    let all_patterns = parse_lines(lines);
    aggregate_all_patterns(&all_patterns, true)
}
