use regex::Regex;
use std::{cmp::Ordering, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

struct CamelPoker {
    cards_rank: Vec<char>,
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [char; 5],
    cards_rank: [u8; 5],
    bid: usize,
    rank: u32,
}

impl CamelPoker {
    fn new() -> CamelPoker {
        // 13 cards
        CamelPoker {
            cards_rank: vec![
                'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
            ],
        }
    }

    fn rank_card(&self, card: char) -> usize {
        for (i, c) in self.cards_rank.iter().enumerate() {
            if card == *c {
                return self.cards_rank.len() - i - 1;
            }
        }
        0
    }

    fn hand(&self, hand: &str, bid: usize) -> Hand {
        let mut cards: [char; 5] = ['\0'; 5];
        let mut cards_rank: [u8; 5] = [0; 5];
        for (i, c) in hand.chars().enumerate() {
            cards[i] = c;
            cards_rank[i] = self.rank_card(c) as u8;
        }
        let rank = self.rank_hand(&cards);
        Hand {
            cards,
            cards_rank,
            bid,
            rank,
        }
    }

    fn rank_hand(&self, cards: &[char; 5]) -> u32 {
        let mut counts = [0; 13];

        let mut joker_counts = 0;
        let mut best_card = 'A';
        let mut max_count = 0;
        for card in cards {
            if *card == 'J' {
                joker_counts = joker_counts + 1;
            } else {
                let i = self.rank_card(*card);
                counts[i] = counts[i] + 1;
                if counts[i] > max_count {
                    best_card = *card;
                    max_count = counts[i];
                }
            }
        }

        if joker_counts > 0 {
            let mut new_cards = cards.clone();
            for c in new_cards.iter_mut() {
                if *c == 'J' {
                    *c = best_card;
                }
            }
            return self.rank_hand(&new_cards);
        }

        //let max_count = *counts.iter().max().unwrap();

        if max_count == 1 {
            return 1; // high card
        } else if max_count == 2 {
            let pairs = counts.iter().filter(|&c| *c == 2).count();
            if pairs == 1 {
                return 2; // one pair
            } else {
                return 3; // two pairs
            }
        } else if max_count == 3 {
            let is_pair = counts.iter().any(|c| *c == 2);
            if is_pair {
                return 5; // full house
            } else {
                return 4; // three of kind
            }
        } else if max_count > 3 {
            return max_count as u32 + 2;
        }
        panic!("max count is is invalid!");
    }
}

impl Hand {
    fn compare_high_cards(&self, other: &Hand) -> Ordering {
        for (left, right) in self.cards_rank.iter().zip(other.cards_rank.iter()) {
            match left.cmp(right) {
                Ordering::Equal => continue,
                other => return other,
            }
        }
        return Ordering::Equal;
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.compare_high_cards(other) == Ordering::Equal
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank_order = self.rank.cmp(&other.rank);
        match rank_order {
            Ordering::Equal => self.compare_high_cards(other),
            other => other,
        }
    }
}

fn solution(input: &str) -> usize {
    let rules = CamelPoker::new();

    let mut all_hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let line: Vec<&str> = line.split_whitespace().collect();
            let hand = line.iter().nth(0).unwrap();
            let bid = usize::from_str(line.iter().nth(1).unwrap()).unwrap();
            rules.hand(hand, bid)
        })
        .collect();

    all_hands.sort();

    let mut sum = 0;
    for (i, hand) in all_hands.iter().enumerate() {
        let global_rank = i + 1;
        let score = global_rank * hand.bid;
        sum = sum + score;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test1.txt");
        let expected_result = 5905;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
