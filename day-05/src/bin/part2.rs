

use rayon::prelude::*;

fn main() {
    let input = include_str!("input.txt");
    let result = solution(input);
    println!("Result: {}", result);
}

#[derive(Debug, Clone, Copy)]
struct Range {
    beg: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy)]
struct Ranges {
    dst: Range,
    src: Range,
}

impl Ranges {
    fn new(dst_beg: usize, src_beg: usize, len: usize) -> Ranges {
        Ranges {
            dst: Range::new(dst_beg, len),
            src: Range::new(src_beg, len),
        }
    }
}

use regex::Regex;
use std::str::FromStr;

impl Range {
    fn new(beg: usize, len: usize) -> Range {
        Range {
            beg,
            end: beg + len,
        }
    }

    fn in_range(&self, num: usize) -> bool {
        num >= self.beg && num < self.end
    }
}

fn parse_seeds(input: &str) -> Vec<usize> {
    let re = Regex::new(r"(\d+)").unwrap();
    re.captures_iter(input)
        .map(|cap| usize::from_str(&cap[1]).unwrap())
        .collect()
}

fn lookup(seed: usize, ranges: &Vec<Ranges>) -> usize {
    // by default, dst == seed
    let dst = seed;
    for &range in ranges {
        if range.src.in_range(seed) {
            return range.dst.beg + (seed - range.src.beg);
        }
    }
    dst
}

fn parse_ranges(input: &str) -> Vec<Ranges> {
    let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let output: Vec<Ranges> = input
        .lines()
        .skip(1)
        .map(|line| {
            let cap = re.captures(line).unwrap();
            let dst = usize::from_str(&cap[1]).unwrap();
            let src = usize::from_str(&cap[2]).unwrap();
            let len = usize::from_str(&cap[3]).unwrap();
            Ranges::new(dst, src, len)
        })
        .collect();
    output
}

fn lookup_all(seed: usize, maps: &Vec<Vec<Ranges>>) -> usize {
    let mut input = seed;
    for i in 0..maps.len() {
        input = lookup(input, &maps[i]);
    }
    input
}

fn solution(input: &str) -> usize {
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    let seeds = parse_seeds(input[0]);
    let mut maps_v = Vec::new();
    let maps = vec![
        "seed-to-soil map:", 
        "soil-to-fertilizer map:", 
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:"
        ];
    for i in 1..=maps.len() {
        maps_v.push(parse_ranges(input[i]));
    }

    let mut m = usize::max_value();
    for i in (0..seeds.len()).step_by(2) {
        let a = seeds[i];
        let b = a + seeds[i+1];
        println!("Processing seeds [{},{})", a, b);
        let n = (a..b).into_par_iter().map(|seed| {
            lookup_all(seed, &maps_v)
        }).min().unwrap();
        println!("Current min={} got={}", m, n);
        if n < m {
            m = n;
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_input = include_str!("test1.txt");
        let expected_result = 46;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
