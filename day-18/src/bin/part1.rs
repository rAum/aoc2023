use regex::Regex;

fn main() {
    let input = include_str!("input");
    let result = solution(input);
    println!("Result: {}", result);
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    fn from_char(dig: &char) -> Dir {
        match dig {
            'U' => Dir::N,
            'D' => Dir::S,
            'R' => Dir::E,
            'L' => Dir::W,
            _ => panic!("Wrong direction"),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Dig {
    dir: Dir,
    len: usize,
    //color: &'a str,
}

type XY = (i64, i64);
fn to_polygon(cmds: Vec<Dig>) -> Vec<XY> {
    let mut v: Vec<XY> = Vec::new();
    let mut pos: XY = (0, 0);
    v.push(pos);
    for cmd in cmds.iter() {
        let n = cmd.len as i64;
        let new_pos = match cmd.dir {
            Dir::N => (pos.0, pos.1 - n),
            Dir::S => (pos.0, pos.1 + n),
            Dir::W => (pos.0 - n, pos.1),
            Dir::E => (pos.0 + n, pos.1),
        };
        pos = new_pos;
        v.push(new_pos);
    }
    v
}

fn calculate_area_and_perimeter(points: &[XY]) -> (i64, i64) {
    let mut area = 0;
    let mut perimeter = 0;

    for i in 0..points.len() - 1 {
        let j = (i + 1) % points.len();
        let x_1 = points[i].0;
        let x_2 = points[j].0;
        let y_1 = points[i].1;
        let y_2 = points[j].1;
        let a = x_1.abs_diff(x_2) as i64;
        let b = y_1.abs_diff(y_2) as i64;
        let seg = (x_1 - x_2).abs() + (y_1 - y_2).abs();
        area += a + b + (x_1 + x_2) * (y_1 - y_2);
        perimeter += seg;
    }

    area = 1 + area.abs() / 2;

    (area, perimeter)
}

fn solution(input: &str) -> i64 {
    let reg = Regex::new(r"(?P<cmd>[A-Z]) (?P<len>\d+) \((?P<color>#[0-9a-fA-F]{6})\)").unwrap();
    let cmds: Vec<Dig> = input
        .lines()
        .map(|line| {
            reg.captures(line)
                .and_then(|cap| {
                    let cmd = cap.name("cmd")?.as_str().chars().next()?;
                    let len = cap.name("len")?.as_str().parse().ok()?;
                    let hexcolor = cap.name("color")?.as_str().to_string();
                    println!("{} {} {}", cmd, len, hexcolor);
                    Some(Dig {
                        dir: Dir::from_char(&cmd),
                        len: len,
                    })
                })
                .unwrap()
        })
        .collect();

    // build polygon
    let poly = to_polygon(cmds);

    let (area, per) = calculate_area_and_perimeter(&poly.as_slice());
    println!("A={} P={}", area, per);
    area + per
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let test_input = r#"R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"#;
        let expected_result = 62;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
