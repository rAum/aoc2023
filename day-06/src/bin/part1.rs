use regex::Regex;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> usize {
    let re = Regex::new(r"(\d+)").unwrap();
    let times_distances: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| usize::from_str(&cap[1]).unwrap())
                .collect()
        })
        .collect();

    let mut wins_product = 0;
    for (time, distance) in times_distances[0].iter().zip(times_distances[1].iter()) {
        println!("{} {}", time, distance);
        let mut wins = 0;
        for hold in 1..*time {
            let max_distance = (time - hold) * hold;
            if max_distance > *distance {
                wins += 1;
            }
        }
        if wins > 0 {
            println!("Wins: {}", wins);
            if wins_product == 0 {
                wins_product = wins;
            } else {
                wins_product = wins_product * wins;
            }
        }
    }
    wins_product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test1.txt");
        let expected_result = 288;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
