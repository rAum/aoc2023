fn main() {
    let input = include_str!("input1.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> String {
    // parse input each line
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let result: i32 = input
        .lines()
        .map(|line| {
            if line.is_empty() {
                return 0;
            }
            // replace strings into chars
            let mut no = Vec::new();
            let mut i = 0;
            let line = line.chars().collect::<Vec<char>>();
            for letter in &line {
                if letter.is_digit(10) {
                    no.push((i, (*letter as u8 - '0' as u8)));
                    i += 1;
                    continue;
                }
                for j in 0..nums.len() {
                    let num_len = nums[j].len();
                    if (i + num_len) > line.len() {
                        continue;
                    }
                    let sub = String::from_iter(&line[i..i + num_len]);
                    if nums[j] == sub {
                        no.push((i, (j as u8) + 1));
                        break;
                    }
                }
                i += 1;
            }

            no.sort_by_key(|a| a.0);

            let a = no.first().unwrap().1;
            let b = no.last().unwrap().1;

            let r =
                a.to_string().parse::<i32>().unwrap() * 10 + b.to_string().parse::<i32>().unwrap();
            println!("{}", r);
            r
        })
        .sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test2.txt");
        let expected_result = "281";

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
