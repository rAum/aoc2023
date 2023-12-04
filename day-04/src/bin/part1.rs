use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}


fn solution(input: &str) -> u32 {
    input.lines().filter_map(|line| -> Option<u32> {
        let card_sequnces: Vec<_> = line.split(":").collect();
        let _card = card_sequnces.iter().nth(0).unwrap();    
        let sequences: Vec<_> = card_sequnces.iter().nth(1).unwrap().split("|").collect();

        let winning_numbers: HashSet<u32> = sequences[0]
        .split_whitespace()
        .map(|txt| -> u32 {
            txt.trim().to_string().parse::<u32>().unwrap()
        }).collect();

        let winning_tickets = sequences[1]
        .split_whitespace()
        .filter_map(|txt| {
            let v = txt.trim().to_string().parse::<u32>().unwrap();
            if winning_numbers.contains(&v) {
                return Some(v)
            }
            None
        }).count();
        let score: u32 = match winning_tickets {
            0 => 0,
            v => 2u32.pow(v as u32 - 1),
        };
        Some(score)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test1.txt");
        let expected_result = 13;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
