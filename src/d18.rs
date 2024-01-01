#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct DigPlan {
    digs: Vec<Dig>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Dig {
    direction: Direction,
    distance: i64,
    color: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_lines(lines: &Vec<String>, convert_hex: bool) -> DigPlan {
    let mut digs: Vec<Dig> = Vec::new();

    for line in lines.iter() {
        let mut split = line.split_whitespace();

        let mut direction = match split.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };

        let mut distance = split.next().unwrap().parse::<i64>().unwrap();
        let color = split
            .next()
            .unwrap()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .to_string();

        if convert_hex {
            distance = i64::from_str_radix(&color[1..color.len() - 1], 16).unwrap();
            direction = match color.chars().last().unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("Invalid direction"),
            };
        }

        digs.push(Dig {
            direction,
            distance,
            color,
        });
    }

    DigPlan { digs }
}

fn process_dig_plan(dig_plan: &DigPlan) -> i64 {
    let (mut x1, mut y1) = (0, 0);

    // Pick's theorem: A = i + b/2 - 1 where A is area, i is number of interior points and b is number of boundary points
    let mut area = 0;
    let mut num_boundary_pts = 0;

    for dig in dig_plan.digs.iter() {
        let (x2, y2) = match dig.direction {
            Direction::Up => (x1 - dig.distance, y1),
            Direction::Down => (x1 + dig.distance, y1),
            Direction::Left => (x1, y1 - dig.distance),
            Direction::Right => (x1, y1 + dig.distance),
        };

        // Shoelace formula where area = 1/2 * (x0*y1 - x1*y0 + ...)
        area += x1 * y2 - x2 * y1;
        num_boundary_pts += dig.distance;

        (x1, y1) = (x2, y2);
    }

    area = area / 2;
    let num_interior_pts = area.abs() - num_boundary_pts / 2 + 1;

    num_interior_pts + num_boundary_pts
}

pub fn p1(lines: &Vec<String>) -> i64 {
    let dig_plan = parse_lines(lines, false);
    process_dig_plan(&dig_plan)
}

pub fn p2(lines: &Vec<String>) -> i64 {
    let dig_plan = parse_lines(lines, true);
    process_dig_plan(&dig_plan)
}
