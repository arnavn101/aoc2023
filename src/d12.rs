use memoize::memoize;

fn parse_lines(lines: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut springs: Vec<String> = Vec::new();
    let mut records: Vec<String> = Vec::new();

    lines.iter().for_each(|line| {
        let mut split = line.split_whitespace();
        springs.push(split.next().unwrap().to_string());
        records.push(split.next().unwrap().to_string());
    });

    (springs, records)
}

#[memoize]
fn find_num_arrangements(spring: String, records: String) -> usize {
    let parsed_records: Vec<usize> = if records.is_empty() {
        Vec::new()
    } else {
        records
            .split(',')
            .map(|s| {
                s.parse::<usize>()
                    .unwrap_or_else(|_| panic!("{} of length {} is not a number", s, s.len()))
            })
            .collect::<Vec<usize>>()
    };
    let parsed_spring: Vec<char> = spring.chars().collect::<Vec<char>>();

    if parsed_spring.is_empty() {
        return parsed_records.is_empty() as usize;
    } else if parsed_records.is_empty() {
        return !parsed_spring.iter().any(|c| *c == '#') as usize;
    } else if parsed_spring.len() < parsed_records.iter().sum::<usize>() + parsed_records.len() - 1
    {
        return 0;
    }

    if parsed_spring[0] == '.' {
        return find_num_arrangements(spring[1..].to_string(), records);
    } else if parsed_spring[0] == '#' {
        let first_record = parsed_records[0];
        for i in 0..first_record {
            if parsed_spring[i] == '.' {
                return 0;
            }
        }
        if first_record < parsed_spring.len() && parsed_spring[first_record] == '#' {
            return 0;
        }

        let s_nxt: String = if first_record < parsed_spring.len() {
            spring[first_record + 1..].to_string()
        } else {
            String::new()
        };
        return find_num_arrangements(
            s_nxt,
            parsed_records[1..]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
        );
    }

    // ? - split into two cases
    let (mut s1, mut s2): (String, String) = (String::from('#'), String::from('.'));
    s1.push_str(&spring[1..]);
    s2.push_str(&spring[1..]);

    return find_num_arrangements(s1, records.clone()) + find_num_arrangements(s2, records.clone());
}

fn dup_spring_and_record(spring: &String, record: &String) -> (String, String) {
    let mut new_spring = spring.clone();
    let mut new_record = record.clone();

    for _ in 0..4 {
        new_spring.push('?');
        new_spring.push_str(&mut spring.clone());

        new_record.push(',');
        new_record.push_str(&mut record.clone());
    }

    (new_spring, new_record)
}

pub fn p1(lines: &Vec<String>) -> usize {
    let (springs, records) = parse_lines(lines);
    springs
        .iter()
        .zip(records.iter())
        .fold(0, |acc, (spring, records)| {
            let cur_arrangements = find_num_arrangements(spring.clone(), records.clone());
            acc + cur_arrangements
        })
}

pub fn p2(lines: &Vec<String>) -> usize {
    let (springs, records) = parse_lines(lines);
    springs
        .iter()
        .zip(records.iter())
        .fold(0, |acc, (spring, records)| {
            let (extended_spring, extended_records) = dup_spring_and_record(spring, records);
            let cur_arrangements = find_num_arrangements(extended_spring, extended_records);
            acc + cur_arrangements
        })
}
