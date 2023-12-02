fn main() {
    let input = include_str!("input1.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> String {
    // parse input each line
    let result: i32 = input
        .lines()
        .map(|line| {
            if line.is_empty() {
                return 0;
            }
            let line = line.chars().collect::<Vec<char>>();

            let length = line.len();

            let mut i = 0;
            let mut a: char = '0';
            let mut b: char = '0';

            while i < length {
                if line[i].is_digit(10) {
                    a = line[i];
                    break;
                }
                i += 1;
            }

            let mut j = length - 1;

            while j > i {
                if line[j].is_digit(10) {
                    b = line[j];
                    break;
                }
                j -= 1;
            }
            if i == j {
                b = a;
            }
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
        let test_input = include_str!("test1.txt");
        let expected_result = "142";

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
