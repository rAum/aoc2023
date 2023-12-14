use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

fn is_spring_matching(seq: &[char], i: usize, spring: i64) -> bool {
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

fn solve(seq: &[char], i: usize, springs: &[i64], cache: &mut Vec<i64>) -> i64 {
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

    let cache_idx = seq.len() * springs.len() + i;
    if cache[cache_idx] != -1 {
        return cache[cache_idx];
    }

    let (spring, rest_springs) = unsafe { springs.split_first().unwrap_unchecked() };
    let mut result = 0;
    if is_spring_matching(seq, i, *spring) {
        let offset = i + *spring as usize + 1;
        result += solve(seq, offset, rest_springs, cache);
    }
    if seq[i] == '?' && i + 1 < seq.len() {
        result += solve(seq, i + 1, springs, cache);
    }

    cache[cache_idx] = result;

    return result;
}

fn count_springs(seq: &Vec<char>, springs: &Vec<i64>) -> i64 {
    let mut cache = Vec::new();
    cache.resize(seq.len() * (springs.len() + 1), -1);
    solve(seq.as_slice(), 0, springs.as_slice(), &mut cache)
}

fn solution(input: &str) -> i64 {
    let parse = Regex::new(r"([#\.\?]+)\s*(.*),*").unwrap();
    input
        .lines()
        .map(|line| {
            let parsed = parse.captures(line).unwrap();
            let seq: Vec<char> = parsed[1].chars().collect();
            let springs = parsed[2]
                .split(',')
                .map(|src| i64::from_str_radix(src, 10).unwrap())
                .collect::<Vec<_>>();
            let mut dup_seq: Vec<char> = seq.clone();
            let mut dup_springs = springs.clone();
            for _ in 0..4 {
                dup_seq.push('?');
                dup_seq.append(&mut seq.clone());
                dup_springs.append(&mut springs.clone());
            }
            count_springs(&dup_seq, &dup_springs)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn test_solution(#[case] input: &str, #[case] answer: i64) {
        let result = solution(input);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_solution2() {
        let input = include_str!("test1.txt");
        let result = solution(input);
        assert_eq!(result, 525152);
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
        #[case] spring: i64,
        #[case] expected: bool,
    ) {
        let input_chars: Vec<char> = input.chars().collect();
        let result = is_spring_matching(input_chars.as_slice(), offset, spring);
        assert_eq!(result, expected);
    }
}
