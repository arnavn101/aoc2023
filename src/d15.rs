#[derive(Debug)]
enum OpType {
    ADD,
    REMOVE,
}

#[derive(Debug)]
struct Op {
    op_type: OpType,
    operands: Vec<String>,
}

fn parse_lines(lines: &Vec<String>) -> Vec<String> {
    lines[0].split(',').map(|s| s.to_string()).collect()
}

fn hash_func(s: &String) -> usize {
    let mut h = 0;
    for c in s.chars() {
        h += c as usize;
        h *= 17;
        h %= 256;
    }
    h
}

fn get_ops(strs: &Vec<String>) -> Vec<Op> {
    let mut ops: Vec<Op> = Vec::new();
    for s in strs {
        let mut op: Op = Op {
            op_type: OpType::ADD,
            operands: Vec::new(),
        };

        if s.ends_with('-') {
            op.op_type = OpType::REMOVE;
            op.operands = vec![s[..s.len() - 1].to_string()];
        } else {
            let split = s.split('=').collect::<Vec<&str>>();
            op.operands = vec![split[0].to_string(), split[1].to_string()];
        }

        ops.push(op);
    }
    ops
}

fn perform_ops(ops: &Vec<Op>) -> usize {
    let mut boxes: [Vec<(String, String)>; 256] = vec![Vec::new(); 256].try_into().unwrap();

    for op in ops {
        match op.op_type {
            OpType::ADD => {
                let h = hash_func(&op.operands[0]);
                let item = (op.operands[0].clone(), op.operands[1].clone());

                if let Some(i) = boxes[h].iter().position(|s| *s.0 == op.operands[0]) {
                    boxes[h][i] = item;
                } else {
                    boxes[h].push(item);
                }
            }
            OpType::REMOVE => {
                let h = hash_func(&op.operands[0]);
                boxes[h].retain(|s| *s.0 != op.operands[0]);
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(|(j, (label, length))| {
                    let l = length
                        .parse::<usize>()
                        .unwrap_or_else(|_| panic!("{} could not be parsed", length));
                    (i + 1) * (j + 1) * l
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn p1(lines: &Vec<String>) -> usize {
    let strs = parse_lines(lines);
    strs.iter().map(|s| hash_func(s)).sum::<usize>()
}

pub fn p2(lines: &Vec<String>) -> usize {
    let strs = parse_lines(lines);
    let ops = get_ops(&strs);
    perform_ops(&ops)
}
