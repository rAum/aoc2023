fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn to_array(rocks: &str) -> Vec<Vec<char>> {
    rocks.lines().map(|line| line.chars().collect()).collect()
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = matrix.first().unwrap().len();
    (0..width)
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect()
}

fn count_mirror_vertical(rocks: &Vec<Vec<char>>) -> usize {
    let h = rocks.len();

    for y in 1..h {
        let top = &rocks[..y];
        let down = &rocks[y..];

        let mut is_valid = true;
        for (a, b) in top.iter().rev().zip(down.iter()) {
            if a.iter().zip(b.iter()).any(|(a, b)| a != b) {
                is_valid = false;
                break;
            }
        }
        if is_valid {
            return top.len();
        }
    }
    0
}

fn count(rocks: Vec<Vec<char>>) -> usize {
    let vertical = count_mirror_vertical(&rocks);
    let horizontal = count_mirror_vertical(&transpose(&rocks));
    100 * vertical + horizontal
}

fn solution(input: &str) -> usize {
    input
        .split("\r\n\r\n")
        .into_iter()
        .map(to_array)
        .map(count)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#;
        let result = solution(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_solution2() {
        let input = include_str!("test1.txt");
        let result = solution(input);
        assert_eq!(result, 405);
    }
}
