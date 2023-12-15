fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn to_array(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = matrix.first().unwrap().len();
    (0..width)
        .map(|i| matrix.iter().map(|row| row[i]).collect())
        .collect()
}

fn print(rocks: &Vec<Vec<char>>) {
    let h = rocks.len();
    let w = rocks.first().unwrap().len();
    for y in 0..h {
        for x in 0..w {
            print!("{}", rocks[y][x]);
        }
        println!();
    }
    println!("------------------");
}

fn slide(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tr = transpose(rocks);
    let h = tr.len();
    let w = tr.first().unwrap().len();

    let mut changed = true;
    while changed {
        changed = false;
        for y in 0..h {
            for x in 1..w {
                if tr[y][x-1] == '.' && tr[y][x] == 'O' {
                    tr[y].swap(x, x - 1);
                    changed = true;
                }
            }
        }
    }
    transpose(&tr)
}

fn solution(input: &str) -> usize {
    let rocks = to_array(input);
    print(&rocks);
    let slided = slide(&rocks);
    print(&slided);
    calc_weight(&slided)
}

fn calc_weight(rocks: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for (i, line) in rocks.iter().rev().enumerate() {
        let n_rocks = line.iter().filter(|c| **c == 'O').count();
        result += (i + 1) * n_rocks;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn test_weight() {
//         let input = r#"OOOO.#.O..
// OO..#....#
// OO..O##..O
// O..#.OO...
// ........#.
// ..#....#.#
// ..O..#.O.O
// ..O.......
// #....###..
// #....#...."#;
//         let result = calc_weight(&to_array(input));
//         assert_eq!(result, 136);
//     }

    #[test]
    fn test_solution() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        let result = solution(input);
        assert_eq!(result, 136);
    }
}
