fn parse_lines(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let mut v: Vec<i32> = Vec::new();
        for num in line.split_whitespace() {
            v.push(num.parse::<i32>().unwrap());
        }
        res.push(v);
    }
    res
}

pub fn get_next_num(v: &Vec<i32>, rev: bool) -> i32 {
    let mut list_seq: Vec<Vec<i32>> = Vec::new();
    list_seq.push(v.clone());

    loop {
        let mut cur_seq: Vec<i32> = Vec::new();
        let cur_v = list_seq.last().unwrap();
        for i in 1..cur_v.len() {
            let diff = cur_v[i] - cur_v[i - 1];
            cur_seq.push(diff);
        }
        list_seq.push(cur_seq.clone());
        if cur_seq.iter().all(|x| *x == 0) {
            break;
        }
    }

    let mut lst_or_fst: i32 = 0;

    if rev {
        for cur_seq in list_seq.iter().rev() {
            lst_or_fst = cur_seq.first().unwrap() - lst_or_fst;
        }
    } else {
        for cur_seq in list_seq.iter() {
            lst_or_fst += cur_seq.last().unwrap();
        }
    }

    lst_or_fst
}

pub fn p1(lines: &Vec<String>) -> i32 {
    let nums = parse_lines(lines);
    nums.iter().map(|v| get_next_num(v, false)).sum()
}

pub fn p2(lines: &Vec<String>) -> i32 {
    let nums = parse_lines(lines);
    nums.iter().map(|v| get_next_num(v, true)).sum()
}
