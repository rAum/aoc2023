use std::collections::{VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

#[derive(Debug)]
struct PipeMaze {
    h: i32,
    w: i32,
    map: Vec<Vec<char>>,
    distmap: Vec<Vec<i32>>,
    start_pos: (i32, i32),
}

impl PipeMaze {
    fn new(input: &str) -> PipeMaze {
        let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let h = map.len() as i32;
        let w = map[0].len() as i32;

        let mut distmap = Vec::new();
        distmap.resize(h as usize, Vec::new());
        for v in distmap.iter_mut() {
            v.resize(w as usize, -2);
        }

        let mut start_pos = (-1, -1);

        for y in 0..h {
            for x in 0..w {
                let v = map[y as usize][x as usize];
                match v {
                    '.' => {
                        distmap[y as usize][x as usize] = -1;
                    }
                    'S' => {
                        start_pos = (y, x);
                        distmap[y as usize][x as usize] = 0;
                    }
                    _ => {}
                }
            }
        }
        PipeMaze {
            h,
            w,
            map,
            distmap,
            start_pos,
        }
    }

    fn connection(pipe: char) -> Option<((i32, i32), (i32, i32))> {
        match pipe {
            '|' => Some(((-1, 0), (1, 0))),
            '-' => Some(((0, -1), (0, 1))),
            'L' => Some(((-1, 0), (0, 1))),
            'J' => Some(((-1, 0), (0, -1))),
            '7' => Some(((1, 0), (0, -1))),
            'F' => Some(((1, 0), (0, 1))),
            _ => None,
        }
    }

    fn find_loop(&mut self) -> i32 {
        let seed = self.start_pos;
        let mut queue = VecDeque::new();

        // add to S only valid pipes as seed points. 
        // Assume pipe is always a valid cycle.
        let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dy, dx) in offsets {
            // pos is S neighbour
            let pos = (seed.0 + dy, seed.1 + dx);
            if pos.0 < 0 || pos.0 >= self.h || pos.1 < 0 || pos.1 >= self.w {
                continue;
            }
            let pipe = self.map[pos.0 as usize][pos.1 as usize];
            let conn = PipeMaze::connection(pipe);
            if let Some((a, b)) = conn {
                let a = (pos.0 + a.0, pos.1 + a.1);
                let b = (pos.0 + b.0, pos.1 + b.1);
                if a == seed || b == seed {
                    queue.push_back((pos, 1));
                }
            }
        }
        let mut max_len = 1;
        while !queue.is_empty() {
            let (pos, dist) = queue.pop_front().unwrap();

            // exit if invalid
            if pos.0 < 0 || pos.0 > self.h {
                continue;
            }
            if pos.1 < 0 || pos.1 > self.w {
                continue;
            }

            let y = pos.0 as usize;
            let x = pos.1 as usize;
            if self.distmap[y][x] > -2 {
                // already visited!
                continue;
            }
            self.distmap[y][x] = dist;
            if dist > max_len {
                max_len = dist;
            }
            let pipe = self.map[y][x];
            let ndist = dist + 1;

            match PipeMaze::connection(pipe) {
                Some((a, b)) => {
                    queue.push_back(((pos.0 + a.0, pos.1 + a.1), ndist));
                    queue.push_back(((pos.0 + b.0, pos.1 + b.1), ndist));
                }
                None => {}
            }
        }
        max_len
    }
}

fn solution(input: &str) -> i32 {
    let mut maze = PipeMaze::new(input);
    maze.find_loop()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let test_input = include_str!("test0.txt");
        let expected_result = 4;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_1() {
        let test_input = include_str!("test1.txt");
        let expected_result = 8;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
