use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug)]
enum ObstacleType {
    RightMirror,
    LeftMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

#[derive(Debug)]
struct Obstacle {
    obstacle_type: ObstacleType,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Layout {
    row_map: HashMap<usize, Vec<Obstacle>>,
    col_map: HashMap<usize, Vec<Obstacle>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Beam {
    direction: Direction,
    x: usize,
    y: usize,
}

fn parse_lines(lines: &Vec<String>) -> (usize, usize, Layout) {
    let (mut row_map, mut col_map) = (HashMap::new(), HashMap::new());
    let (m, n) = (lines.len(), lines[0].len());

    // init row_map and col_map
    for i in 0..m {
        row_map.insert(i, Vec::new());
    }
    for j in 0..n {
        col_map.insert(j, Vec::new());
    }

    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let obstacle_type = match c {
                '/' => ObstacleType::RightMirror,
                '\\' => ObstacleType::LeftMirror,
                '|' => ObstacleType::VerticalSplitter,
                '-' => ObstacleType::HorizontalSplitter,
                _ => continue,
            };
            row_map.get_mut(&x).unwrap().push(Obstacle {
                obstacle_type,
                x,
                y,
            });
            col_map.get_mut(&y).unwrap().push(Obstacle {
                obstacle_type,
                x,
                y,
            });
        }
    }

    (m, n, Layout { row_map, col_map })
}

fn simulate_beam(
    m: usize,
    n: usize,
    layout: &Layout,
    initial_beam: Beam,
) -> HashSet<(usize, usize)> {
    let mut beams: Vec<Beam> = vec![initial_beam];
    let mut visited: HashSet<Beam> = HashSet::new();
    let mut is_beginning = true;

    while !beams.is_empty() {
        let mut new_beams: Vec<Beam> = Vec::new();

        for beam in beams.iter() {
            if visited.contains(beam) {
                continue;
            }

            let (dir, x, y) = (beam.direction, beam.x, beam.y);

            let incoming_obs: Vec<&Obstacle> = match dir {
                Direction::Up => layout.col_map[&y]
                    .iter()
                    .filter(|&o| if is_beginning { o.x <= x } else { o.x < x })
                    .collect(),
                Direction::Down => layout.col_map[&y]
                    .iter()
                    .filter(|&o| if is_beginning { o.x >= x } else { o.x > x })
                    .collect(),
                Direction::Left => layout.row_map[&x]
                    .iter()
                    .filter(|&o| if is_beginning { o.y <= y } else { o.y < y })
                    .collect(),
                Direction::Right => layout.row_map[&x]
                    .iter()
                    .filter(|&o| if is_beginning { o.y >= y } else { o.y > y })
                    .collect(),
            };

            is_beginning = false;

            let mut last_pt = match dir {
                Direction::Up => (0, y),
                Direction::Down => (m - 1, y),
                Direction::Left => (x, 0),
                Direction::Right => (x, n - 1),
            };

            if !incoming_obs.is_empty() {
                // obs is closest obstacle
                let obs = incoming_obs
                    .iter()
                    .min_by_key(|&o| match dir {
                        Direction::Up => x - o.x,
                        Direction::Down => o.x - x,
                        Direction::Left => y - o.y,
                        Direction::Right => o.y - y,
                    })
                    .unwrap();

                let mut new_beam = Beam {
                    direction: dir,
                    x: obs.x,
                    y: obs.y,
                };
                last_pt = (new_beam.x, new_beam.y);

                match obs.obstacle_type {
                    ObstacleType::RightMirror => match dir {
                        Direction::Right => new_beam.direction = Direction::Up,
                        Direction::Left => new_beam.direction = Direction::Down,
                        Direction::Up => new_beam.direction = Direction::Right,
                        Direction::Down => new_beam.direction = Direction::Left,
                    },
                    ObstacleType::LeftMirror => match dir {
                        Direction::Right => new_beam.direction = Direction::Down,
                        Direction::Left => new_beam.direction = Direction::Up,
                        Direction::Up => new_beam.direction = Direction::Left,
                        Direction::Down => new_beam.direction = Direction::Right,
                    },
                    ObstacleType::VerticalSplitter => {
                        if dir == Direction::Right || dir == Direction::Left {
                            new_beam.direction = Direction::Up;
                            new_beams.push(Beam {
                                direction: Direction::Down,
                                x: obs.x,
                                y: obs.y,
                            });
                        }
                    }
                    ObstacleType::HorizontalSplitter => {
                        if dir == Direction::Up || dir == Direction::Down {
                            new_beam.direction = Direction::Right;
                            new_beams.push(Beam {
                                direction: Direction::Left,
                                x: obs.x,
                                y: obs.y,
                            });
                        }
                    }
                }

                new_beams.push(new_beam);
            } else {
                // no obstacle, beam goes to the end
            }

            // mark all as true in visited (except the last point if it's an obstacle)
            match dir {
                Direction::Up => {
                    for i in if !incoming_obs.is_empty() {
                        last_pt.0 + 1..x + 1
                    } else {
                        last_pt.0..x + 1
                    } {
                        visited.insert(Beam {
                            direction: dir,
                            x: i,
                            y: y,
                        });
                    }
                }
                Direction::Down => {
                    for i in if !incoming_obs.is_empty() {
                        x..last_pt.0
                    } else {
                        x..last_pt.0 + 1
                    } {
                        visited.insert(Beam {
                            direction: dir,
                            x: i,
                            y: y,
                        });
                    }
                }
                Direction::Left => {
                    for j in if !incoming_obs.is_empty() {
                        last_pt.1 + 1..y + 1
                    } else {
                        last_pt.1..y + 1
                    } {
                        visited.insert(Beam {
                            direction: dir,
                            x: x,
                            y: j,
                        });
                    }
                }
                Direction::Right => {
                    for j in if !incoming_obs.is_empty() {
                        y..last_pt.1
                    } else {
                        y..last_pt.1 + 1
                    } {
                        visited.insert(Beam {
                            direction: dir,
                            x: x,
                            y: j,
                        });
                    }
                }
            }
        }

        beams = new_beams;
    }

    visited.into_iter().map(|b| (b.x, b.y)).collect()
}

#[allow(dead_code)]
fn print_visited(m: usize, n: usize, visited: &HashSet<Beam>) {
    let mut s = String::new();
    for i in 0..m {
        for j in 0..n {
            if vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .iter()
            .any(|&dir| {
                visited.contains(&Beam {
                    direction: dir,
                    x: i,
                    y: j,
                })
            }) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("{}", s);
}

fn get_beams_from_all_dirs(m: usize, n: usize) -> Vec<Beam> {
    let mut beams: Vec<Beam> = Vec::new();

    for i in 0..m {
        // left to right
        beams.push(Beam {
            direction: Direction::Right,
            x: i,
            y: 0,
        });

        // right to left
        beams.push(Beam {
            direction: Direction::Left,
            x: i,
            y: n - 1,
        });
    }

    for j in 0..n {
        // up to down
        beams.push(Beam {
            direction: Direction::Down,
            x: 0,
            y: j,
        });

        // down to up
        beams.push(Beam {
            direction: Direction::Up,
            x: m - 1,
            y: j,
        });
    }

    beams
}

pub fn p1(lines: &Vec<String>) -> usize {
    let (m, n, layout) = parse_lines(lines);
    let visited = simulate_beam(
        m,
        n,
        &layout,
        Beam {
            direction: Direction::Right,
            x: 0,
            y: 0,
        },
    );
    visited.iter().count()
}

pub fn p2(lines: &Vec<String>) -> usize {
    let (m, n, layout) = parse_lines(lines);
    get_beams_from_all_dirs(m, n)
        .iter()
        .map(|b| simulate_beam(m, n, &layout, b.clone()).iter().count())
        .max()
        .unwrap()
}
