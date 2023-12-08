use regex::Regex;
use std::collections::HashMap;

use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn find_len(start: &str, walk: &Vec<char>, road: &HashMap<String, (String, String)>) -> usize {
    let mut pos = road.get_key_value(start).unwrap();
    let mut i = 0;
    loop {
        if pos.0.ends_with('Z') {
            println!("Reached destination {} -> {} in {} steps", start, pos.0, i);
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

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn solution(input: &str) -> usize {
    let v: Vec<&str> = input.lines().collect();
    let walk = v[0].chars().collect::<Vec<char>>();

    let r = Regex::new(r"^(\w+)\s*=\s*\((\w+),\s*(\w+)\)$").unwrap();
    let mut start = Vec::new();
    let road = v
        .iter()
        .skip(2)
        .map(|line| {
            let cap = r.captures(line).unwrap();
            let pos = cap[1].to_string();
            let left = cap[2].to_string();
            let right = cap[3].to_string();

            if pos.ends_with('A') {
                start.push(pos.clone());
            }

            (pos, (left, right))
        })
        .collect::<HashMap<_, _>>();

    let seeds = start
        .iter()
        .map(|start| road.get_key_value(start).unwrap())
        .collect::<Vec<_>>();

    let cycles_len = (0..seeds.len())
        .into_par_iter()
        .map(|i| -> usize { find_len(seeds[i].0, &walk, &road) })
        .collect::<Vec<usize>>();

    let lcm = cycles_len.iter().fold(1, |acc, &b| acc * b / gcd(acc, b));
    lcm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test3.txt");
        let expected_result = 6;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
