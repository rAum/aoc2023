use std::collections::{HashMap, VecDeque};
use std::thread;
use std::time::Duration;

fn main() {
    let input = include_str!("input");
    let result = solution(input);
    println!("Result: {}", result);
}

#[derive(PartialEq, Clone, Copy)]
enum Dir {
    Top = 1,
    Down = 2,
    Right = 4,
    Left = 8,
}

fn dir_to_v(dir: Dir) -> (i32, i32) {
    match dir {
        Dir::Top => (-1, 0),
        Dir::Down => (1, 0),
        Dir::Right => (0, 1),
        Dir::Left => (0, -1),
    }
}

fn v_to_dir(v: (i32, i32)) -> Dir {
    match v {
        (-1, 0) => Dir::Top,
        (1, 0) => Dir::Down,
        (0, 1) => Dir::Right,
        (0, -1) => Dir::Left,
        _ => panic!("Wrong vector!"),
    }
}

#[inline(always)]
fn add(pos: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    (pos.0 + dir.0, pos.1 + dir.1)
}

fn solution(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let h = grid.len();
    let w = grid.first().unwrap().len();

    let top_down = (0..w).map(|i| simulate(&grid, (0, i as i32), Dir::Down)).max().unwrap();
    let down_top = (0..w).map(|i| simulate(&grid, ((h - 1) as i32, i as i32), Dir::Top)).max().unwrap();
    let left_right = (0..h).map(|i| simulate(&grid, (i as i32, 0), Dir::Right)).max().unwrap();
    let right_left = (0..h).map(|i| simulate(&grid, (i as i32, (w - 1) as i32), Dir::Left)).max().unwrap();
    
    vec![top_down, down_top, left_right, right_left].into_iter().max().unwrap()
}

fn simulate(grid: &Vec<Vec<char>>, start_pos: (i32, i32), dir: Dir) -> usize {
    let h = grid.len() as i32;
    let w = grid.first().unwrap().len() as i32;

    let mut visited = HashMap::new();

    let mut q = VecDeque::new();
    q.push_back((start_pos, dir_to_v(dir)));

    let mut n = 0;
    while !q.is_empty() {
        let (pos, dir) = q.pop_back().unwrap();

        if pos.0 < 0 || pos.0 >= w || pos.1 < 0 || pos.1 >= h {
            continue;
        }

        // keep track if we have visited field
        // from a given direction
        let d = v_to_dir(dir);

        if let Some(v) = visited.get_mut(&pos) {
            if *v & (d as u32) != 0 {
                // we already ended on the same field
                // coming from the same direction
                continue;
            } else {
                *v = (*v | (d as u32)) as u32;
            }
        } else {
            // new field
            n += 1;
            visited.insert(pos, d as u32);
        }

        let field = grid[pos.0 as usize][pos.1 as usize];

        if field == '.'
            || (field == '-' && (d == Dir::Left || d == Dir::Right))
            || (field == '|' && (d == Dir::Top || d == Dir::Down))
        {
            q.push_back((add(pos, dir), dir));
        } else if field == '\\' {
            let new_dir = match d {
                Dir::Top => dir_to_v(Dir::Left),
                Dir::Down => dir_to_v(Dir::Right),
                Dir::Left => dir_to_v(Dir::Top),
                Dir::Right => dir_to_v(Dir::Down),
            };
            let new_pos = add(pos, new_dir);
            q.push_back((new_pos, new_dir));
        } else if field == '/' {
            let new_dir = match d {
                Dir::Top => dir_to_v(Dir::Right),
                Dir::Down => dir_to_v(Dir::Left),
                Dir::Left => dir_to_v(Dir::Down),
                Dir::Right => dir_to_v(Dir::Top),
            };
            let new_pos = add(pos, new_dir);
            q.push_back((new_pos, new_dir));
        } else if field == '-' && (d == Dir::Top || d == Dir::Down) {
            let left = dir_to_v(Dir::Left);
            q.push_back((add(pos, left), left));
            let right = dir_to_v(Dir::Right);
            q.push_back((add(pos, right), right));
        } else if field == '|' && (d == Dir::Left || d == Dir::Right) {
            let up = dir_to_v(Dir::Top);
            q.push_back((add(pos, up), up));
            let down = dir_to_v(Dir::Down);
            q.push_back((add(pos, down), down));
        } else {
            panic!("Incorrect situation! case not handled...");
        }
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        let answer = 51;
        assert_eq!(solution(input), answer);
    }
}
