use std::collections::HashMap;

pub fn p1(all_lines: &Vec<String>) -> i32 {
    all_lines
        .iter()
        .map(|line| -> i32 {
            let (mut first_char, mut last_char) = (-1, -1);

            for char in line.chars() {
                if char.is_digit(10) {
                    let parsed = char.to_digit(10).unwrap() as i32;
                    if first_char == -1 {
                        first_char = parsed;
                    }
                    last_char = parsed;
                }
            }

            last_char + 10 * first_char
        })
        .sum()
}

pub fn p2(all_lines: &Vec<String>) -> i32 {
    let v: Vec<String> = all_lines
        .iter()
        .map(|line| -> String {
            let ht = HashMap::from([
                ("one", 1),
                ("two", 2),
                ("three", 3),
                ("four", 4),
                ("five", 5),
                ("six", 6),
                ("seven", 7),
                ("eight", 8),
                ("nine", 9),
            ]);

            let mut s = String::new();
            let mut indices: HashMap<usize, i32> = HashMap::new();

            ht.iter().for_each(|(&w, &v)| {
                line.match_indices(w).for_each(|(i, _)| {
                    indices.insert(i, v);
                });
            });

            let mut i = 0;
            let line_chars: Vec<char> = line.chars().collect();

            while i < line.len() {
                if indices.contains_key(&i) {
                    let v = indices.get(&i).unwrap();
                    s.push_str(&v.to_string());
                } else {
                    s.push(*line_chars.get(i).unwrap());
                }
                i += 1;
            }

            s
        })
        .collect();

    p1(&v)
}
