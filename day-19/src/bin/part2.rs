use std::{collections::HashMap, path::Display};

use regex::Regex;

fn main() {
    let input = include_str!("input");
    let result = solution(input);
    println!("Result: {}", result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cmd {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl From<usize> for Cmd {
    fn from(item: usize) -> Self {
        match item {
            0 => Cmd::X,
            1 => Cmd::M,
            2 => Cmd::A,
            3 => Cmd::S,
            _ => panic!("Invalid value"),
        }
    }
}

// can be removed
impl Into<usize> for Cmd {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    low: [i64; 4],
    high: [i64; 4],
}

impl std::fmt::Display for PartRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..self.low.len()).for_each(|value| {
            f.write_fmt(format_args!(
                "{:?}=[{},{}] ",
                Cmd::from(value),
                self.low[value],
                self.high[value]
            ));
        });
        Ok(())
    }
}

impl PartRange {
    fn new() -> Self {
        Self {
            low: [1; 4],
            high: [4000; 4],
        }
    }

    fn volume(&self) -> u64 {
        self.low
            .iter()
            .zip(self.high.iter())
            // adding +1 as we assume [a, b] range
            .map(|(a, b)| b.abs_diff(*a) + 1)
            .product()
    }

    fn contains(&self, which: Cmd, value: i64) -> bool {
        let l = self.low[which as usize];
        let h = self.high[which as usize];
        l <= value && value < h
    }

    fn split_to_gt(&self, which: Cmd, value: i64) -> Self {
        // we divide [low, high) v[cmd] > value
        // to get (value, high)
        let mut new_low = self.low.clone();
        new_low[which as usize] = value + 1;
        Self {
            low: new_low,
            high: self.high,
        }
    }

    fn split_to_lt(&self, which: Cmd, value: i64) -> Self {
        // we divide into [low, high) v[cmd] < value
        // to get [low, value)
        let mut new_high = self.high.clone();
        new_high[which as usize] = value - 1;
        Self {
            low: self.low,
            high: new_high,
        }
    }

    /// Splits into (ACCEPTED, INVALID)
    fn split(&self, cmd: &Cmd, rule: &Rule) -> Option<(Self, Self)> {
        match rule {
            Rule::Greater(cmd, v, _) => {
                let h = self.split_to_gt(*cmd, *v); // > v
                let l = self.split_to_lt(*cmd, v + 1); // <= v
                Some((h, l))
            }
            Rule::Less(cmd, v, _) => {
                let h = self.split_to_gt(*cmd, v - 1); // >= v
                let l = self.split_to_lt(*cmd, *v); // < v
                Some((l, h))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Decision {
    Accept,
    Reject,
    GoTo(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rule {
    Accept,
    Reject,
    GoTo(String),
    Less(Cmd, i64, Decision),
    Greater(Cmd, i64, Decision),
}

#[derive(Debug)]
struct Workflows {
    seq: Vec<Vec<Rule>>,
    names: Vec<String>,
    rules: HashMap<String, usize>,
}

impl Workflows {
    fn to_id(&self, name: &str) -> usize {
        *self.rules.get(name).unwrap()
    }
}

fn to_cat_index(letter: &str) -> usize {
    match letter {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => unreachable!(),
    }
}

fn apply_rules_to_range(range: &PartRange, workflows: &Workflows) -> u64 {
    let mut ranges = Vec::new();
    ranges.push((*range, Rule::GoTo("in".to_string())));

    let mut result = 0;

    while !ranges.is_empty() {
        let (range, rule): (PartRange, Rule) = ranges.pop().unwrap();

        match rule {
            Rule::GoTo(target) => {
                let id = workflows.to_id(&target);
                let curr_rules = &workflows.seq[id];
                let name = &workflows.names[id];

                let mut curr_range = range.clone();
                println!("Checking {:#?}: {}", name, curr_range);

                for r in curr_rules.iter() {
                    println!("Analyzing rule {:#?}", r);
                    if curr_range.volume() == 0 {
                        println!("!!!!");
                        break;
                    }
                    match r {
                        Rule::Reject => {
                            println!("Rejected: {}", curr_range);
                            break;
                        }
                        Rule::Accept => {
                            result += curr_range.volume();
                            break;
                        }
                        Rule::Greater(cmd, value, dec) => {
                            if let Some((acc, rej)) = curr_range.split(cmd, &r) {
                                match dec {
                                    Decision::Accept => {
                                        result += acc.volume();
                                        println!("Add {:?}>{} -> {}", cmd, value, acc);
                                        curr_range = rej;
                                    }
                                    Decision::Reject => {
                                        curr_range = rej;
                                    }
                                    Decision::GoTo(target) => {
                                        println!("{} => {}", acc, target);
                                        ranges.push((acc, Rule::GoTo(target.clone())));
                                        curr_range = rej;
                                    }
                                }
                            } else {
                                unreachable!()
                            }
                        }
                        Rule::Less(cmd, value, dec) => {
                            if let Some((acc, rej)) = curr_range.split(cmd, &r) {
                                match dec {
                                    Decision::Accept => {
                                        result += acc.volume();
                                        println!("Add {:?}>{} -> {}", cmd, value, acc);
                                        curr_range = rej;
                                    }
                                    Decision::Reject => {
                                        curr_range = rej;
                                    }
                                    Decision::GoTo(target) => {
                                        println!("{} => {}", acc, target);
                                        ranges.push((acc, Rule::GoTo(target.clone())));
                                        curr_range = rej;
                                    }
                                }
                            } else {
                                unreachable!();
                            }
                        }
                        other_rule => {
                            println!("{:?} {}", other_rule, curr_range);
                            ranges.push((curr_range, other_rule.clone()));
                            break;
                        }
                    }
                }
                println!("Done with {}", name);
            }
            _ => {
                unreachable!()
            }
        }
    }
    result
}

impl Workflows {
    fn new(input: &str) -> Self {
        let mut rules = HashMap::new();
        let mut seq = Vec::new();
        let mut names = Vec::new();

        let re = Regex::new(r"(?P<name>\w+)\{(?P<cmds>.+)\}").unwrap();
        let rule_re =
            Regex::new(r"(?P<cmd>[xmas])(?P<op>[<>])(?P<value>\d+):(?P<target>.+)").unwrap();

        input.lines().for_each(|line| {
            let caps = re.captures(line).unwrap();
            let name = &caps["name"];
            let ruleset: Vec<Rule> = caps["cmds"]
                .split(",")
                .map(|r| {
                    let cap = rule_re.captures(r);
                    let rule = match cap {
                        None => match r {
                            "A" => Rule::Accept,
                            "R" => Rule::Reject,
                            other => Rule::GoTo(other.to_string()),
                        },
                        Some(x) => {
                            let cat: Cmd = to_cat_index(&x["cmd"]).into();
                            let is_greater = &x["op"] == ">";
                            let value: i64 = x["value"].parse().unwrap();
                            let target = match &x["target"] {
                                "A" => Decision::Accept,
                                "R" => Decision::Reject,
                                other => Decision::GoTo(other.to_string()),
                            };

                            match is_greater {
                                true => Rule::Greater(cat, value, target),
                                false => Rule::Less(cat, value, target),
                            }
                        }
                    };
                    rule
                })
                .collect();

            rules.insert(name.to_string(), seq.len());
            seq.push(ruleset);
            names.push(name.to_string());
        });

        Workflows { seq, rules, names }
    }
}

fn solution(input: &str) -> u64 {
    let mut in_rules_parts = input.split("\n\n");
    let rules = in_rules_parts.next().unwrap();
    let rules = Workflows::new(rules);

    let initial_range = PartRange::new();
    let result = apply_rules_to_range(&initial_range, &rules);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_volume() {
        let range = PartRange::new();
        assert_eq!(range.volume(), (4000 as u64).pow(4));
    }

    #[test]
    fn test_range_split_gt() {
        let range = PartRange::new();
        let h = range.split_to_gt(Cmd::M, 1000); // > 1000 so 3k elem
        let l = range.split_to_lt(Cmd::M, 1000 + 1); // <= 1000

        assert_eq!(range.volume(), l.volume() + h.volume());
        assert_eq!(l.volume(), (4000 as u64).pow(3) * 1000);
        assert_eq!(h.volume(), (4000 as u64).pow(3) * 3000);
    }

    #[test]
    fn test_range_split1() {
        let range = PartRange::new();
        if let Some((l, h)) = range.split(&Cmd::X, &Rule::Greater(Cmd::X, 123, Decision::Accept)) {
            assert_eq!(l.volume() + h.volume(), range.volume());
        }
    }

    #[test]
    fn test_range_split2() {
        let range = PartRange::new();
        if let Some((l, h)) = range.split(&Cmd::X, &Rule::Less(Cmd::X, 123, Decision::Accept)) {
            assert_eq!(l.volume() + h.volume(), range.volume());
        }
    }

    #[test]
    fn test_range_split_lt() {
        let range = PartRange::new();
        let l = range.split_to_lt(Cmd::M, 1001); // < 1001 so 1k elem
        let h = range.split_to_gt(Cmd::M, 1000); // >= 1000

        println!("{}", range);

        assert_eq!(range.volume(), l.volume() + h.volume());
        assert_eq!(l.volume(), (4000 as u64).pow(3) * 1000);
        assert_eq!(h.volume(), (4000 as u64).pow(3) * 3000);
    }

    #[test]
    fn test_0() {
        let test_input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

"#;
        let expected_result = 167409079868000;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
