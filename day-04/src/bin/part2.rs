use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> usize {
    let n_games = input.lines().count();
    let mut winners = vec![0; n_games];

    input.lines().enumerate().for_each(|(gameid, line)| {
        let card_sequnces: Vec<_> = line.split(":").collect();
        let _card = card_sequnces.iter().nth(0).unwrap();
        let sequences: Vec<_> = card_sequnces.iter().nth(1).unwrap().split("|").collect();

        let winning_numbers: HashSet<u32> = sequences[0]
            .split_whitespace()
            .map(|txt| -> u32 { txt.trim().to_string().parse::<u32>().unwrap() })
            .collect();

        let n_winning_tickets = sequences[1]
            .split_whitespace()
            .filter_map(|txt| {
                let v = txt.trim().to_string().parse::<u32>().unwrap();
                if winning_numbers.contains(&v) {
                    return Some(v);
                }
                None
            })
            .count();

        winners[gameid] = n_winning_tickets;
    });

    for i in (0..winners.len()).rev() {
        if winners[i] == 0 {
            continue;
        }

        let win_tickets = winners[i];
        let new_winners = &winners[(i + 1)..(i + 1 + win_tickets as usize)];
        winners[i] = winners[i] + new_winners.iter().sum::<usize>();
    }

    winners.iter().sum::<usize>() + winners.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test1.txt");
        let expected_result = 30;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
