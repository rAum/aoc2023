use std::{cmp::max, collections::HashMap};

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}


fn parse_line(line: &str) -> (usize, usize, usize, usize) {
    // line has format Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    let game_rounds: Vec<&str> = line.split(":").collect();
    let game_num = game_rounds[0]
        .split_whitespace().nth(1).unwrap().trim().parse::<usize>().unwrap();
    let all_rounds: Vec<&str> = game_rounds[1].split(";").collect();

    let mut parsed_rounds: Vec<Vec<(usize, usize)>> = Vec::new();
    for round in all_rounds {
        let parsed: Vec<(usize, usize)> = round.split(",").map(|num_col| {
            let n_color = num_col.trim().split(" ").collect::<Vec<&str>>();
            let n : usize = n_color[0].parse::<usize>().unwrap();
            let color: usize = match n_color[1] {
                "red" => 0,
                "blue" => 1,
                "green" => 2,
                _ => panic!("Unknown color"),
            };
            (n, color)
        }).collect();

        // for v in parsed.iter() {
        //     println!("Parsed: {:?}", v);
        // }
        parsed_rounds.push(parsed);
    }

    let mut colors = HashMap::new();
    for i in 0..3 {
        colors.insert(i, 0);
    }
    
    // pick max color appearing in rounds
    for (n, color) in parsed_rounds.into_iter().flatten() {
        let color_count = colors.entry(color).or_default();
        *color_count = max(*color_count, n);
    }

    println!("Max colors for game {}: {:?}", game_num, colors);

    let r = *colors.get(&0).expect("red should be present");
    let b = *colors.get(&1).expect("blue should be present");
    let g = *colors.get(&2).expect("green should be present");

    (r, b, g, game_num)
}

fn solution(input: &str) -> usize {
    // red, blue, green
    let limit = (12, 13, 14);

    let result = input
        .lines()
        .map(parse_line)
        .filter_map(|game| {
            if game.0 > limit.0 || game.1 > limit.1 || game.2 > limit.2 {
                return None;
            }
            Some(game.3)
        }).inspect(|game_n| {
            println!("Game {} is okay", game_n);
        }).sum();
        result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test1.txt");
        let expected_result = 8;
        
        let result = solution(test_input);
        dbg!(result);
        assert_eq!(result, expected_result);
    }
}
