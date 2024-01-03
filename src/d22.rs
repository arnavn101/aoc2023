#[derive(Clone, Debug, PartialEq, Eq)]
struct Brick {
    c1: (i32, i32, i32),
    c2: (i32, i32, i32),
}

fn parse_lines(lines: &Vec<String>) -> Vec<Brick> {
    let mut bricks = Vec::new();
    for line in lines {
        let mut parts = line.split('~');
        let mut c1 = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap());
        let mut c2 = parts
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<i32>().unwrap());
        bricks.push(Brick {
            c1: (c1.next().unwrap(), c1.next().unwrap(), c1.next().unwrap()),
            c2: (c2.next().unwrap(), c2.next().unwrap(), c2.next().unwrap()),
        });
    }
    bricks
}

pub fn p1(lines: &Vec<String>) -> i32 {
    let bricks = parse_lines(lines);
    println!("{:?}", bricks);
    0
}

pub fn p2(lines: &Vec<String>) -> i32 {
    0
}
