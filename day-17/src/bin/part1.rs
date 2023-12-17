use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("input");
    let result = solution(input);
    println!("Result: {}", result);
}

fn print(grid: &Vec<Vec<u32>>) {
    let h = grid.len();
    let w = grid.first().unwrap().len();
    for y in 0..h {
        for x in 0..w {
            print!("{}", grid[y][x]);
        }
        println!();
    }
    println!("--------------");
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    N,
    S,
    W,
    E,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn mov(&self, dir: Dir, len: usize) -> Vec2 {
        match dir {
            Dir::N => Vec2 {
                x: self.x,
                y: self.y - len,
            },
            Dir::S => Vec2 {
                x: self.x,
                y: self.y + len,
            },
            Dir::W => Vec2 {
                x: self.x - len,
                y: self.y,
            },
            Dir::E => Vec2 {
                x: self.x + len,
                y: self.y,
            },
        }
    }

    fn safe_mov(&self, dir: Dir, len: usize, w: usize, h: usize) -> Option<Vec2> {
        match dir {
            Dir::N => {
                if self.y < len {
                    return None;
                };
                Some(Vec2 {
                    x: self.x,
                    y: self.y - len,
                })
            }
            Dir::S => {
                if self.y + len >= h {
                    return None;
                };
                Some(Vec2 {
                    x: self.x,
                    y: self.y + len,
                })
            }
            Dir::W => {
                if self.x < len {
                    return None;
                };
                Some(Vec2 {
                    x: self.x - len,
                    y: self.y,
                })
            }
            Dir::E => {
                if self.x + len >= w {
                    return None;
                }
                Some(Vec2 {
                    x: self.x + len,
                    y: self.y,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct Node {
    pos: Vec2,
    dir: Dir,
    len: usize,
}

fn neighbours_in_dir(pos: Vec2, dir: Dir, w: usize, h: usize) -> impl Iterator<Item = Node> {
    (1..=3).filter_map(move |len| {
        let Some(new_pos) = pos.safe_mov(dir, len, w, h) else {
            return None;
        };
        Some(Node {
            pos: new_pos,
            dir,
            len,
        })
    })
}

fn all_neighbours(pos: Vec2, w: usize, h: usize) -> impl Iterator<Item = Node> {
    let dirs = [Dir::S, Dir::E, Dir::N, Dir::W];
    dirs.into_iter()
        .map(move |dir| neighbours_in_dir(pos, dir, w, h))
        .flatten()
}

fn find_path(costs: &Vec<Vec<u32>>) -> u32 {
    let start_pos = Vec2 { x: 0, y: 0 };
    let h = costs.len();
    let w = costs.first().unwrap().len();
    let end_pos = (h as u32 - 1, w as u32 - 1);

    let mut q = BinaryHeap::new();

    // add initial moves
    let starting_points = all_neighbours(start_pos, w, h);


    // q.push(start_pos);

    // let mut visited = HashMap::with_capacity(w * h);

    // while !q.is_empty() {
    //     let curr = q.pop().unwrap();

    //     // get neighbours...

    //     if curr == end_pos {
    //         return
    //     }
    // }
    0
}

fn solution(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| (c as u8 - b'0') as u32).collect())
        .collect();
    let mut res = grid.clone();
    let h = grid.len() as i32;
    let w = grid.first().unwrap().len() as i32;

    print(&grid);
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0, 0, 6)]
    #[case(1, 0, 7)]
    #[case(4, 4, 3 * 4)]
    #[case(9, 9, 6)]
    fn test_neighbours(#[case] x: usize, #[case] y: usize, #[case] expected_neighbours_count: usize) {
        let all_pos: Vec<_> = all_neighbours(Vec2 { x, y }, 10, 10).collect();
        assert_eq!(all_pos.len(), expected_neighbours_count);
    }

    #[test]
    fn test_solution() {
        let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
        let answer = 102;
        assert_eq!(solution(input), answer);
    }
}
