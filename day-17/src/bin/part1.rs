use std::time::Duration;
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    thread,
};

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

impl Dir {
    fn as_char(&self) -> char {
        match self {
            Dir::N => '^',
            Dir::S => 'v',
            Dir::W => '<',
            Dir::E => '>',
        }
    }
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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
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

fn opposite_dir(dir: Dir) -> Dir {
    match dir {
        Dir::N => Dir::S,
        Dir::S => Dir::N,
        Dir::W => Dir::E,
        Dir::E => Dir::W,
    }
}

/// Calculates directly cost for a given movement
fn calc_cost(costs: &Vec<Vec<u32>>, node: &Node) -> u32 {
    let steps = node.len;
    let dir = opposite_dir(node.dir);

    let movement_cost = (0..steps)
        .map(|l| {
            let curr = node.pos.mov(dir, l);
            costs[curr.y][curr.x]
        })
        .sum();
    movement_cost
}

#[derive(Debug, Ord, Eq)]
struct CostNode {
    cost: u32,
    node: Node,
}

impl PartialEq for CostNode {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl PartialOrd for CostNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // seriously, is there nothing better?
        match self.cost.partial_cmp(&other.cost) {
            Some(std::cmp::Ordering::Less) => Some(std::cmp::Ordering::Greater),
            Some(std::cmp::Ordering::Greater) => Some(std::cmp::Ordering::Less),
            Some(std::cmp::Ordering::Equal) => Some(std::cmp::Ordering::Equal),
            None => None,
        }
    }
}

fn is_valid_move(curr: &Node, next: &Node) -> bool {
    // do not go back nor do not go the same direction
    if curr.dir == opposite_dir(next.dir) {
        return false;
    }
    if curr.dir != next.dir {
        return true;
    }
    false
}

fn find_path(costs: &Vec<Vec<u32>>) -> u32 {
    let start_pos = Vec2 { x: 0, y: 0 };
    let h = costs.len();
    let w = costs.first().unwrap().len();
    let end_pos = Vec2 { x: w - 1, y: h - 1 };

    let mut frontier = BinaryHeap::new();
    let mut node_cost: HashMap<Node, u32> = HashMap::with_capacity(w * h * 3 * 4);
    let mut visited: HashSet<Node> = HashSet::with_capacity(w * h * 3 * 4);

    // add initial moves
    for step in all_neighbours(start_pos, w, h) {
        let cost = calc_cost(costs, &step);
        frontier.push(CostNode {
            cost: calc_cost(costs, &step),
            node: step,
        });
        node_cost.insert(step, cost);
    }

    while !frontier.is_empty() {
        let curr = frontier.pop().unwrap();

        // we already processed given
        if visited.contains(&curr.node) {
            continue;
        }
        // mark this node as visited
        visited.insert(curr.node);
        // have we reached goal?

        if curr.node.pos == end_pos {
            let total_cost = *node_cost.get(&curr.node).unwrap();
            println!("Reached goal {:#?}", curr.node);
            return total_cost;
        }

        let neighbours =
            all_neighbours(curr.node.pos, w, h).filter(|n| is_valid_move(&curr.node, n));

        let curr_total_cost = *node_cost.get(&curr.node).unwrap();

        for next in neighbours {
            let next_total_move_cost = calc_cost(costs, &next) + curr_total_cost;

            if let Some(old_cost) = node_cost.get_mut(&next) {
                if *old_cost > next_total_move_cost {
                    *old_cost = next_total_move_cost;
                    frontier.push(CostNode {
                        cost: next_total_move_cost,
                        node: next,
                    });
                } else {
                    // nothing to do  - longer path.
                }
            } else {
                // infinity node
                frontier.push(CostNode {
                    cost: next_total_move_cost,
                    node: next,
                });
                node_cost.insert(next, next_total_move_cost);
            }
        }
    }
    panic!("Failed to reach end goal :(");
}

fn solution(input: &str) -> usize {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| (c as u8 - b'0') as u32).collect())
        .collect();
    find_path(&grid) as usize
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
    fn test_neighbours(
        #[case] x: usize,
        #[case] y: usize,
        #[case] expected_neighbours_count: usize,
    ) {
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
