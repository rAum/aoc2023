use regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn extrapolate(seq: &Vec<i32>) -> i32 {
    let mut stack = Vec::new();
    let mut reduced = false;
    stack.push(seq.clone());
    while !reduced {
        let prev = stack.last().unwrap();
        let mut s = Vec::new();
        s.resize(prev.len() - 1, 0);
        for i in 0..prev.len() - 1 {
            s[i] = prev[i + 1] - prev[i];
        }
        if s.iter().all(|x| *x == 0) {
            reduced = true;
        }
        stack.push(s);
    }

    let mut acc = 0;
    for i in (0..stack.len() - 1).rev() {
        acc = stack[i].last().unwrap() + acc;
    }
    acc
}

fn solution(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let seq = line
                .split_whitespace()
                .map(|v| i32::from_str_radix(v, 10).unwrap())
                .collect::<Vec<_>>();

            extrapolate(&seq)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let test_input = include_str!("test1.txt");
        let expected_result = 114;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
