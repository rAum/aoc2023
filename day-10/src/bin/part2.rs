use std::{collections::VecDeque, sync::atomic::fence};

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
    fence: Vec<Vec<bool>>,
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

        let mut fence = Vec::new();
        fence.resize(h as usize, Vec::new());
        for v in fence.iter_mut() {
            v.resize(w as usize, false);
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
            fence,
        }
    }

    fn print_fence(&self) {
        println!();
        for y in 0..self.h {
            for x in 0..self.w {
                let f = match self.fence[y as usize][x as usize] {
                    true => self.map[y as usize][x as usize],
                    false => '.',
                };
                print!("{}", f);
            }
            println!();
        }
        println!();
    }

    fn print_map(&self) {
        println!();
        for y in 0..self.h {
            for x in 0..self.w {
                print!("{}", self.map[y as usize][x as usize]);
            }
            println!();
        }
        println!();
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

    fn build_loop_fence(&mut self) {
        let seed = self.start_pos;
        self.fence[seed.0 as usize][seed.1 as usize] = true;
        let mut queue = VecDeque::new();

        // add to S only valid pipes as seed points.
        // Assume pipe is always a valid cycle.
        let mut s_connectom = Vec::new();
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
                let off = (a, b);
                let a = (pos.0 + a.0, pos.1 + a.1);
                let b = (pos.0 + b.0, pos.1 + b.1);
                if a == seed {
                    queue.push_back((pos, 1));
                    s_connectom.push((-off.0 .0, -off.0 .1));
                } else if b == seed {
                    queue.push_back((pos, 1));
                    s_connectom.push((-off.1 .0, -off.1 .1));
                }
            }
        }

        let mut s_replacement = 'S';
        for pipe in ['|', '-', 'L', 'J', '7', 'F'] {
            let (a, b) = PipeMaze::connection(pipe).unwrap();
            if s_connectom[0] == a && s_connectom[1] == b {
                s_replacement = pipe;
                break;
            } else if s_connectom[0] == b && s_connectom[1] == a {
                s_replacement = pipe;
                break;
            }
        }
        println!("----> S => {}", s_replacement);

        self.map[seed.0 as usize][seed.1 as usize] = s_replacement;
        self.print_map();

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
            let pipe = self.map[y][x];
            let ndist = dist + 1;

            match PipeMaze::connection(pipe) {
                Some((a, b)) => {
                    queue.push_back(((pos.0 + a.0, pos.1 + a.1), ndist));
                    queue.push_back(((pos.0 + b.0, pos.1 + b.1), ndist));
                    self.fence[y][x] = true;
                }
                None => {}
            }
        }
    }

    fn get_inside_area(&mut self) -> i32 {
        let mut area = 0;

        for y in 0..self.h {
            let mut is_inside = false;

            for x in 0..self.w {
                let is_fence = self.fence[y as usize][x as usize];

                if is_fence {
                    let ftype = self.map[y as usize][x as usize];
                    // hmm it seems only these kind of connectors
                    // are needed to be checked..
                    match ftype {
                        '|' => is_inside = !is_inside,
                        'L' => is_inside = !is_inside,
                        'J' => is_inside = !is_inside,
                        _ => (),
                    }
                } else {
                    if is_inside {
                        area = area + 1;
                    }
                };
            }
        }

        area
    }
}

fn solution(input: &str) -> i32 {
    let mut maze = PipeMaze::new(input);
    maze.build_loop_fence();
    maze.print_fence();
    maze.get_inside_area()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let test_input = include_str!("test2_0.txt");
        let expected_result = 4;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_1() {
        let test_input = include_str!("test2_1.txt");
        let expected_result = 4;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_2() {
        let test_input = include_str!("test2_2.txt");
        let expected_result = 10;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_3() {
        let test_input = include_str!("test2_3.txt");
        let expected_result = 8;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
