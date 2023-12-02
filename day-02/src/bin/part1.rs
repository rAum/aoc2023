fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}


fn solve(line: &str) -> Option<usize> {
    // red, blue, green
    let limit: Vec<usize> = vec![12, 14, 13];

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

        for (n, color) in parsed.iter() {
            if *n > limit[*color] {
                return None;
            }
        }
        parsed_rounds.push(parsed);
    }
    Some(game_num)
}

fn solution(input: &str) -> usize {
    let result = input
        .lines()
        .filter_map(solve)
        .inspect(|game_n| {
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
