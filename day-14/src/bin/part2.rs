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

fn slide(rocks: &Vec<Vec<char>>, dir: bool, do_transpose: bool) -> Vec<Vec<char>> {
    let mut tr = if do_transpose { transpose(rocks) } else { rocks.clone() };
    let h = tr.len();
    let w = tr.first().unwrap().len();

    let mut changed = true;
    while changed {
        changed = false;
        for y in 0..h {
            if dir {
                for x in 1..w {
                    if tr[y][x - 1] == '.' && tr[y][x] == 'O' {
                        tr[y].swap(x, x - 1);
                        changed = true;
                    }
                }
            } else {
                for x in 0..w - 1 {
                    if tr[y][x + 1] == '.' && tr[y][x] == 'O' {
                        tr[y].swap(x, x + 1);
                        changed = true;
                    }
                }
            }
        }
    }
    if do_transpose { transpose(&tr) } else { tr }
}

fn run_one_cycle(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let north = slide(&rocks, true, true);
    let west = slide(&north, true, false);
    let south = slide(&west, false, true);
    let east = slide(&south, false, false);
    return east;
}

fn solution(input: &str) -> usize {
    let rocks = to_array(input);
    print(&rocks);
    let slided = run_one_cycle(&rocks);
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

    #[test]
    fn test_cycle() {
        let input = to_array(r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#);

        let answer = to_array(r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."#);
        let result = run_one_cycle(&input);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_2cycle() {
        let input = to_array(r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#);

        let answer = to_array(r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"#);
        let result = run_one_cycle(&run_one_cycle(&input));
        assert_eq!(result, answer);
    }

    #[test]
    fn test_3cycle() {
        let input = to_array(r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#);

        let answer = to_array(r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"#);
        let result = run_one_cycle(&run_one_cycle(&run_one_cycle(&input)));
        assert_eq!(result, answer);
    }
}
