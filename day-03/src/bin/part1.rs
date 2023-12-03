use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> u32 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let h = map.len();
    let w = map[0].len();

    let mut symbols_pos: HashSet<(usize, usize)> = HashSet::new();
    let mut numbers = Vec::new();

    // well, in rust you cannot iterate over -1..=1 range..
    let offsets: Vec<i32> = vec![-1, 0, 1];

    for y in 0..h {
        let mut x = 0;
        while x < w {
            let v = map[y][x];
            if v != '.' && !v.is_numeric() {
                for dx in &offsets {
                    for dy in &offsets {
                        let px = (x as i32 + dx).clamp(0, w as i32);
                        let py = (y as i32 + dy).clamp(0, h as i32);
                        symbols_pos.insert((px as usize, py as usize));
                    }
                }
                x += 1;
            } else if v.is_numeric() {
                // parse number
                let mut n = v.to_digit(10).unwrap() as u32;
                let mut positions = vec![(x, y)];
                positions.push((x, y));
                x += 1;
                while x < w && map[y][x].is_numeric() {
                    n = n * 10 + map[y][x].to_digit(10).unwrap() as u32;
                    positions.push((x, y));
                    x += 1;
                }
                numbers.push((n, positions));
            } else {
                x += 1;
            }
        }
    }

    let result = numbers
        .into_iter()
        .filter_map(|(value, positions)| {
            for pos in positions {
                if symbols_pos.contains(&pos) {
                    return Some(value);
                }
            }
            None
        })
        .sum();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test1.txt");
        let expected_result = 4361;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
