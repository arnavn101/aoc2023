use std::collections::{HashMap, HashSet};

use binary_heap_plus::BinaryHeap;

fn parse_lines(lines: &Vec<String>) -> Vec<Vec<i32>> {
    let mut grid: Vec<Vec<i32>> = Vec::new();

    for line in lines.iter() {
        let mut row: Vec<i32> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as i32);
        }
        grid.push(row);
    }

    grid
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Crucible {
    position: Position,
    same_dir_times: i32,
    heat_loss: i32,
}

fn iterate_grid(grid: &Vec<Vec<i32>>, min_consec: i32, max_consec: i32) -> (i32, Vec<Crucible>) {
    let mut heap = BinaryHeap::new_by(
        |c1: &(Crucible, Vec<Crucible>), c2: &(Crucible, Vec<Crucible>)| {
            c2.0.heat_loss.cmp(&c1.0.heat_loss)
        },
    );
    let mut vis: HashSet<(Position, i32)> = HashSet::new();

    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let all_dirs = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    for start_dir in vec![Direction::Right, Direction::Down] {
        let cruc = Crucible {
            position: Position {
                x: 0,
                y: 0,
                dir: start_dir,
            },
            same_dir_times: 0,
            heat_loss: 0,
        };
        heap.push((cruc, vec![cruc]));
    }

    while !heap.is_empty() {
        let (crucible, prev_cruc) = heap.pop().unwrap();
        let (x, y) = (crucible.position.x, crucible.position.y);
        let (cur_dir, cur_same_dir_times) = (crucible.position.dir, crucible.same_dir_times);

        let vis_key = (crucible.position, cur_same_dir_times);
        if vis.contains(&vis_key) {
            continue;
        }
        vis.insert(vis_key);

        if x == m - 1 && y == n - 1 {
            return (crucible.heat_loss, prev_cruc);
        }

        // consider_dirs will contain all dirs that are not the opposite of the current dir
        let mut consider_dirs = all_dirs.clone();
        consider_dirs.retain(|&d| d != cur_dir.opposite());

        // remove all dirs apart from current if we have not gone for min_consec
        if cur_same_dir_times < min_consec {
            consider_dirs.retain(|&d| d == cur_dir);
        }

        // remove the current dir if we have been going in the same dir for max_consec
        if cur_same_dir_times == max_consec {
            consider_dirs.retain(|&d| d != cur_dir);
        }

        for dir in consider_dirs {
            let (dx, dy) = match dir {
                Direction::Up => (-1, 0),
                Direction::Down => (1, 0),
                Direction::Left => (0, -1),
                Direction::Right => (0, 1),
            };

            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || nx >= m || ny < 0 || ny >= n {
                continue;
            }

            let heat_loss = crucible.heat_loss + grid[nx as usize][ny as usize];
            let same_dir_times = if dir == cur_dir {
                cur_same_dir_times + 1
            } else {
                1
            };

            let cruc = Crucible {
                position: Position { x: nx, y: ny, dir },
                same_dir_times,
                heat_loss,
            };
            let mut pth = prev_cruc.clone();
            pth.push(cruc);

            heap.push((cruc, pth));
        }
    }

    (-1, vec![])
}

#[allow(dead_code)]
fn print_grid(crucibles: &Vec<Crucible>, grid: &Vec<Vec<i32>>) {
    let mut vis: HashMap<(i32, i32), Direction> = HashMap::new();
    for cruc in crucibles {
        vis.insert((cruc.position.x, cruc.position.y), cruc.position.dir);
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if vis.contains_key(&(i as i32, j as i32)) {
                let dir = vis.get(&(i as i32, j as i32)).unwrap();
                match dir {
                    Direction::Up => print!("^"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                    Direction::Right => print!(">"),
                }
            } else {
                print!("{}", grid[i][j]);
            }
        }
        println!();
    }
}

pub fn p1(lines: &Vec<String>) -> i32 {
    let grid = parse_lines(lines);
    iterate_grid(&grid, 0, 3).0
}

pub fn p2(lines: &Vec<String>) -> i32 {
    let grid = parse_lines(lines);
    iterate_grid(&grid, 4, 10).0
}
