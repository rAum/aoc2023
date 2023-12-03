fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> u32 {
    // parse input each line
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test1.txt");
        let expected_result = "4361";

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
