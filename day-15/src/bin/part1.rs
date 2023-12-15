fn main() {
    let input = include_str!("input");
    let result = solution(input);
    println!("Result: {}", result);
}

fn solution(input: &str) -> usize {
    let bytes: Vec<u8> = input.trim_end_matches('\n').bytes().collect();
    let ranges = bytes.split(|&v| v == b',');
    let mut result: usize = 0;
    for range in ranges {
        result += hash(range) as usize;
    }
    result
}

fn hash(range: &[u8]) -> usize {
    let mut hash = 0;

    for c in range.iter() {
        hash += *c as usize;
        hash *= 17;
        hash = hash % 256;
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_hash(#[case] input: &str, #[case] answer: usize) {
        assert_eq!(hash(input.as_bytes()), answer);
    }

    #[test]
    fn test_solution() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let answer = 1320;
        assert_eq!(solution(input), answer);
    }
}
