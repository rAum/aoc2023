use rayon::prelude::*;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> usize {
    let re = Regex::new(r"(\d+)").unwrap();
    let times_distances: Vec<Vec<String>> = input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| String::from(&cap[1]))
                .collect()
        })
        .collect();

    let time = usize::from_str(&times_distances[0].join("")).expect("Time number");
    let distance = usize::from_str(&times_distances[1].join("")).expect("Dist number");

    (1..time)
        .into_par_iter()
        .filter_map(|hold| {
            let max_distance = (time - hold) * hold;
            if max_distance > distance {
                return Some(1);
            }
            None
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test1.txt");
        let expected_result = 71503;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
