fn parse_engine(all_lines: &Vec<String>) -> Vec<Vec<(i32, i32, String)>> {
    let mut v: Vec<Vec<(i32, i32, String)>> = Vec::new();
    let length_line = all_lines.get(0).unwrap().len() as i32;

    for line in all_lines {
        let mut cur_num = String::new();
        let mut vv: Vec<(i32, i32, String)> = Vec::new();

        for (ch_idx, ch) in line.chars().enumerate() {
            let is_digit = ch.is_digit(10);
            let ch_num = ch_idx as i32;

            if is_digit {
                cur_num.push(ch);
            } else if ch != '.' {
                vv.push((ch_num, ch_num, ch.to_string()))
            }

            if !is_digit || ch_num == length_line - 1 {
                if !cur_num.is_empty() {
                    vv.push((ch_num - cur_num.len() as i32, ch_num - 1, cur_num.clone()))
                }

                cur_num.clear();
            }
        }

        v.push(vv);
    }

    v
}

fn is_digit(s: &String) -> bool {
    return s.chars().nth(0).unwrap_or('f').is_digit(10);
}

pub fn p1(all_lines: &Vec<String>) -> i32 {
    let mut tot_sum = 0;
    let empty_vec: Vec<(i32, i32, String)> = Vec::new();

    let v = parse_engine(all_lines);
    let n = v.len();

    for i in 0..n {
        let v_prev = match i {
            0 => &empty_vec,
            _ => v.get(i - 1).unwrap(),
        };
        let v_nxt = match i {
            last_elem if last_elem == n - 1 => &empty_vec,
            _ => v.get(i + 1).unwrap(),
        };
        let v_cur = v.get(i).unwrap();

        for (start, end, s) in v_cur.iter() {
            if is_digit(s) {
                let num: i32 = s.parse().unwrap();
                for (start2, end2, s2) in v_prev.iter().chain(v_cur.iter()).chain(v_nxt.iter()) {
                    if !is_digit(s2) && s2 != "." {
                        let mut break_out = false;
                        for incr in -1..2 {
                            for pts in vec![start, end] {
                                if (pts + incr) >= *start2 && (pts + incr) <= *end2 {
                                    tot_sum += num;
                                    break_out = true;
                                    break;
                                }
                            }
                            if break_out {
                                break;
                            }
                        }
                        if break_out {
                            break;
                        }
                    }
                }
            }
        }
    }

    tot_sum
}

pub fn p2(all_lines: &Vec<String>) -> i32 {
    let mut tot_sum = 0;
    let empty_vec: Vec<(i32, i32, String)> = Vec::new();

    let v = parse_engine(all_lines);
    let n = v.len();

    for i in 0..n {
        let v_prev = match i {
            0 => &empty_vec,
            _ => v.get(i - 1).unwrap(),
        };
        let v_nxt = match i {
            last_elem if last_elem == n - 1 => &empty_vec,
            _ => v.get(i + 1).unwrap(),
        };
        let v_cur = v.get(i).unwrap();

        for (start, end, s) in v_cur.iter() {
            let mut digit_cnt = 0;
            let mut list_digits: Vec<i32> = Vec::new();

            if s == "*" {
                for (start2, end2, s2) in v_prev.iter().chain(v_cur.iter()).chain(v_nxt.iter()) {
                    let mut break_out = false;

                    if is_digit(s2) {
                        let num: i32 = s2.parse().unwrap();

                        for incr in -1..2 {
                            for pts in vec![start, end] {
                                if (pts + incr) >= *start2 && (pts + incr) <= *end2 {
                                    list_digits.push(num);
                                    digit_cnt += 1;
                                    break_out = true;
                                    break;
                                }
                            }
                            if break_out || digit_cnt > 2 {
                                break;
                            }
                        }
                    }
                }

                if digit_cnt == 2 {
                    tot_sum += list_digits.get(0).unwrap() * list_digits.get(1).unwrap();
                }
            }
        }
    }

    tot_sum
}
