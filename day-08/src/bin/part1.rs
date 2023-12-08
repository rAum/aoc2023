use regex::Regex;
use std::{collections::HashMap};

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> usize {
    let v: Vec<&str> = input.lines().collect();
    let walk = v[0].chars().collect::<Vec<char>>();

    let r = Regex::new(r"^(\w+)\s*=\s*\((\w+),\s*(\w+)\)$").unwrap();
    let road = v
        .iter()
        .skip(2)
        .map(|line| {
            let cap = r.captures(line).unwrap();
            let pos = cap[1].to_string();
            let left = cap[2].to_string();
            let right = cap[3].to_string();
            (pos, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let mut i = 0;
    let start = "AAA";
    let end = "ZZZ";
    let mut pos = road.get_key_value(start).unwrap();
    loop {
        if pos.0 == end {
            println!("Reached destination");
            return i;
        }
        let cmd = walk[i % walk.len()];
        let next = match cmd {
            'L' => &pos.1 .0,
            'R' => &pos.1 .1,
            _ => panic!("Invalid direction"),
        };
        pos = road.get_key_value(next).unwrap();
        i = i + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = include_str!("test1.txt");
        let expected_result = 2;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_2() {
        let test_input = include_str!("test2.txt");
        let expected_result = 6;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
