use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn is_spring_matching(seq: &[char], i: usize, spring: i32) -> bool {
    let end = i + spring as usize;
    if end > seq.len() {
        return false;
    }
    if seq[i..end].iter().any(|v| *v == '.') {
        return false;
    }
    if seq.len() == end {
        return true;
    }
    return seq[end] != '#';
}

fn solve(seq: &[char], i: usize, springs: &[i32]) -> i32 {
    if springs.is_empty() {
        if i >= seq.len() {
            return 1;
        }
        if seq[i..].iter().any(|v| *v == '#') {
            return 0;
        }
        return 1;
    }
    //println!("{:?}| {} |{:?} {:?}", &seq[..i], seq[i], &seq[i+1..], springs);

    let mut i = i;
    while i < seq.len() {
        if seq[i] == '.' {
            i += 1;
            continue;
        }
        break;
    }

    if i >= seq.len() {
        return 0;
    }

    let (spring, rest_springs) = unsafe { springs.split_first().unwrap_unchecked() };
    let mut result = 0;
    if is_spring_matching(seq, i, *spring) {
        let offset = i + *spring as usize + 1;
        result += solve(seq, offset, rest_springs);
    }
    if seq[i] == '?' && i + 1 < seq.len() {
        result += solve(seq, i + 1, springs);
    }

    return result;
}

fn count_springs(seq: &Vec<char>, springs: &Vec<i32>) -> i32 {
    solve(seq.as_slice(), 0, springs.as_slice())
}

fn solution(input: &str) -> i32 {
    let parse = Regex::new(r"([#\.\?]+)\s*(.*),*").unwrap();
    input
        .lines()
        .map(|line| {
            let parsed = parse.captures(line).unwrap();
            let seq: Vec<char> = parsed[1].chars().collect();
            let springs = parsed[2]
                .split(',')
                .map(|src| i32::from_str_radix(src, 10).unwrap())
                .collect::<Vec<_>>();
            count_springs(&seq, &springs)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_solution(#[case] input: &str, #[case] answer: i32) {
        let result = solution(input);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_solution2() {
        let input = include_str!("test1.txt");
        let result = solution(input);
        assert_eq!(result, 21);
    }

    #[rstest]
    #[case(".??..??...?##.", 0, 2, false)]
    #[case(".??..??...?##.", 1, 2, true)]
    #[case(".??..??...?##.", 2, 2, false)]
    #[case("?##.", 0, 2, false)]
    #[case("?##.", 1, 2, true)]
    #[case("?##.", 0, 3, true)]
    #[case("?##", 0, 3, true)]
    #[case("?##", 2, 3, false)]
    fn matching(
        #[case] input: String,
        #[case] offset: usize,
        #[case] spring: i32,
        #[case] expected: bool,
    ) {
        let input_chars: Vec<char> = input.chars().collect();
        let result = is_spring_matching(input_chars.as_slice(), offset, spring);
        assert_eq!(result, expected);
    }
}
