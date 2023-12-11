use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input, 1_000_000);
    println!("Result: {}", result);
}

fn dist_manh((y, x): &(i64, i64), (y2, x2): &(i64, i64)) -> i64 {
    let nx = (x2 - x).abs();
    let ny = (y2 - y).abs();
    nx + ny
}

fn solution(input: &str, space_dist: i64) -> i64 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let h = map.len();
    let w = map[0].len();

    let mut galaxies = Vec::new();

    for y in 0..h {
        for x in 0..w {
            let v = map[y][x];
            match v {
                '#' => galaxies.push((y as i64, x as i64)),
                _ => (),
            }
        }
    }

    // Why space_dist -1 ? well it seems there is one-by-off error
    // if space_dist is added, probably because distance already includes
    // initial galaxy offset
    let galaxy_x = galaxies.iter().map(|(_, x)| *x).collect::<HashSet<_>>();
    let mut warp_x = vec![0; w];
    for i in 0..warp_x.len() {
        if i as i64 - 1 >= 0 {
            warp_x[i] += warp_x[i - 1];
        }
        if !galaxy_x.contains(&(i as i64)) {
            warp_x[i] = warp_x[i] + space_dist - 1;
        }
    }

    let galaxy_y = galaxies.iter().map(|(y, _)| *y).collect::<HashSet<_>>();
    let mut warp_y = vec![0; h];
    for i in 0..warp_y.len() {
        if i as i64 - 1 >= 0 {
            warp_y[i] += warp_y[i - 1];
        }
        if !galaxy_y.contains(&(i as i64)) {
            warp_y[i] = warp_y[i] + space_dist - 1;
        }
    }

    let corrected = galaxies
        .iter()
        .map(|(y, x)| (y + warp_y[*y as usize], x + warp_x[*x as usize]))
        .collect::<Vec<_>>();

    let distances = corrected
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| corrected[i + 1..].iter().map(move |b| dist_manh(&a, b)))
        .collect::<Vec<_>>();

    distances.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = include_str!("test1.txt");
        let expected_result = 1030;

        let result = solution(test_input, 10);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_2() {
        let test_input = include_str!("test1.txt");
        let expected_result = 8410;

        let result = solution(test_input, 100);

        assert_eq!(result, expected_result);
    }
}
