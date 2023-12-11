use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> i64 {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let h = map.len();
    let w = map[0].len();

    let mut galaxies = Vec::new();

    for y in 0..h {
        for x in 0..w {
            let v = map[y][x];
            match v {
                '#' => galaxies.push((y as u32, x as u32)),
                _ => (),
            }
        }
    }

    let x_nonwarp = galaxies.iter().map(|(y, x)| *x).collect::<HashSet<_>>();
    let x_warped: HashSet<u32> = HashSet::from_iter(0..w as u32)
        .difference(&x_nonwarp)
        .cloned()
        .collect();

    let y_nonwarp = galaxies.iter().map(|(y, x)| *y).collect::<HashSet<_>>();
    let y_warped: HashSet<u32> = HashSet::from_iter(0..h as u32)
        .difference(&y_nonwarp)
        .cloned()
        .collect();

    let mut warp_x = vec![0; w];
    let mut warp_y = vec![0; h];

    for i in 0..warp_x.len() {
        if x_warped.contains(&(i as u32)) {
            warp_x[i] = warp_x[i] + 1;
        }
        if i as i64 - 1 >= 0 {
            warp_x[i] += warp_x[i - 1];
        }
    }

    for i in 0..warp_y.len() {
        if y_warped.contains(&(i as u32)) {
            warp_y[i] = warp_y[i] + 1;
        }
        if i as i64 - 1 >= 0 {
            warp_y[i] += warp_y[i - 1];
        }
    }

    let corrected = galaxies
        .iter()
        .map(|(y, x)| (y + warp_y[*y as usize], x + warp_x[*x as usize]))
        .collect::<Vec<_>>();

    let distances = corrected
        .iter()
        .enumerate()
        .flat_map(|(i, &(y, x))| {
            corrected[i + 1..].iter().map(move |(y2, x2)| {
                let nx = (*x2 as i64 - x as i64).abs();
                let ny = (*y2 as i64 - y as i64).abs();
                nx + ny
            })
        })
        .collect::<Vec<_>>();

    distances.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = include_str!("test1.txt");
        let expected_result = 374;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
