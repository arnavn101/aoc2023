use std::collections::HashMap;

fn parse_lines(lines: &Vec<String>) -> (Vec<char>, HashMap<String, (String, String)>) {
    let instructions: Vec<char> = lines[0].clone().chars().collect();
    let mut ht = HashMap::new();

    for i in 2..lines.len() {
        let split: Vec<String> = lines[i].split_whitespace().map(|s| s.to_string()).collect();
        ht.insert(
            split[0].clone(),
            (split[2][1..4].to_string(), split[3][..3].to_string()),
        );
    }

    return (instructions, ht);
}

fn calculate_steps_p1(instructions: &Vec<char>, ht: &HashMap<String, (String, String)>) -> usize {
    let mut steps = 0;
    let mut current = String::from("AAA");

    let mut instr_ptr = 0;
    let n = instructions.len();

    while !current.eq(&String::from("ZZZ")) {
        let (left, right) = ht.get(&current).unwrap();

        if instructions[instr_ptr] == 'L' {
            current = left.clone();
        } else {
            current = right.clone();
        }

        instr_ptr = (instr_ptr + 1) % n;
        steps += 1;
    }

    steps
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(nums: Vec<usize>) -> usize {
    let mut lcm = nums[0];
    for i in 1..nums.len() {
        lcm = lcm * nums[i] / gcd(lcm, nums[i]);
    }
    lcm
}

fn calculate_steps_p2(instructions: &Vec<char>, ht: &HashMap<String, (String, String)>) -> usize {
    let all_loops = ht
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| compute_loops(instructions, ht, s.clone()))
        .collect::<Vec<usize>>();
    lcm(all_loops)
}

fn compute_loops(
    instructions: &Vec<char>,
    ht: &HashMap<String, (String, String)>,
    start: String,
) -> usize {
    let mut steps = 0;

    let mut instr_ptr = 0;
    let n = instructions.len();

    let mut visited: HashMap<(String, usize), usize> = HashMap::new();
    let mut current = start.clone();
    let mut list_end_steps: Vec<usize> = Vec::new();

    loop {
        let (left, right) = ht.get(&current).unwrap();
        steps += 1;

        match instructions[instr_ptr] {
            'L' => {
                current = left.clone();
            }
            'R' => {
                current = right.clone();
            }
            _ => panic!("Invalid instruction"),
        }

        if current.ends_with('Z') {
            list_end_steps.push(steps);
        }

        let cur_state = (current.clone(), instr_ptr);

        if visited.contains_key(&cur_state) {
            return list_end_steps[0];
        }

        visited.insert(cur_state, steps);
        instr_ptr = (instr_ptr + 1) % n;
    }
}

pub fn p1(lines: &Vec<String>) -> usize {
    let (instructions, ht) = parse_lines(lines);
    calculate_steps_p1(&instructions, &ht)
}

pub fn p2(lines: &Vec<String>) -> usize {
    let (instructions, ht) = parse_lines(lines);
    calculate_steps_p2(&instructions, &ht)
}
