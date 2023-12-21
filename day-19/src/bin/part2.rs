use std::collections::HashMap;

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
struct MachinePart {
    data: [i64; 4],
}

#[derive(Debug, Clone, Copy)]
struct PartRange {
    low: MachinePart,
    high: MachinePart, // high is open set
}

impl PartRange {
    fn new() -> Self {
        let low_data = [1; 4];
        let high_data = [4000 + 1; 4];
        Self {
            low: MachinePart { data: low_data },
            high: MachinePart { data: high_data },
        }
    }

    fn volume(&self) -> u64 {
        self.low
            .data
            .iter()
            .zip(self.high.data.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .product()
    }

    fn contains(&self, which: Cmd, value: i64) -> bool {
        let l = self.low.data[which as usize];
        let h = self.high.data[which as usize];
        l <= value && value < h
    }

    fn split_to_gt(&self, which: Cmd, value: i64) -> Self {
        // we divide [low, high)
        // to get (value, high)
        let mut low = self.low.data.clone();
        low[which as usize] = value + 1;
        Self {
            low : MachinePart { data: low },
            high: MachinePart { data: self.high.data.clone() },
        }
    }

    fn split_to_lt(&self, which: Cmd, value: i64) -> Self {
        // we divide into [low, high)
        // to get [low, value)
        let mut high = self.high.data.clone();
        high[which as usize] = value;
        Self {
            low: MachinePart { data: self.low.data.clone() },
            high: MachinePart { data: high },
        }
    }

    /// Splits range into two ranges, given split point
    fn split(&self, which: Cmd, value: i64) -> (Self, Self) {
        let mut low = self.low.data.clone();
        low[which as usize] = value;
        
        let mut high = self.high.data.clone();
        high[which as usize] = value;

        // TODO: fix it without one-by-off error!
        let lower = Self {
            low: MachinePart {
                data: self.low.data,
            },
            high: MachinePart {
                data: high,
            },
        };
        let upper = Self {
            low: MachinePart {
                data: low,
            },
            high: MachinePart {
                data: self.high.data,
            },
        };
        (lower, upper)
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

fn apply(part: &MachinePart, rule: &Rule) -> Rule {
    match rule {
        any => any.clone(),
    }
}

fn apply_rules(part: &MachinePart, workflows: &Workflows) -> bool {
    let mut id = workflows.to_id("in");
    while id < workflows.seq.len() {
        let curr_rules = &workflows.seq[id];

        for rule in curr_rules {
            match apply(part, rule) {
                Rule::Accept => return true,
                Rule::Reject => return false,
                Rule::GoTo(target) => {
                    id = workflows.to_id(&target);
                    break;
                }
                Rule::Greater(cmd, value, decision) => {
                    if part.data[cmd as usize] > value {
                        match decision {
                            Decision::Accept => return true,
                            Decision::Reject => return false,
                            Decision::GoTo(target) => {
                                id = workflows.to_id(&target);
                                break;
                            }
                        }
                    }
                }
                Rule::Less(cmd, value, decision) => {
                    if part.data[cmd as usize] < value {
                        match decision {
                            Decision::Accept => return true,
                            Decision::Reject => return false,
                            Decision::GoTo(target) => {
                                id = workflows.to_id(&target);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn apply_range(range: &PartRange, rules: &Vec<Rule>) -> Vec<(PartRange, Rule)> {
    rules.iter().filter_map(|rule| {
        match rule {
            Rule::Reject => None,
            other => Some((*range, other.clone())),
        }
    }).collect()
}

fn apply_rules_to_range(range: &PartRange, workflows: &Workflows) -> u64 {
    let mut ranges = Vec::new();
    ranges.push((*range, Rule::GoTo("in".to_string())));

    let mut result = 0;

    while !ranges.is_empty() {
        let (range, rule): (PartRange, Rule) = ranges.pop().unwrap();

        match rule {
            Rule::Accept => {
                println!("Adding range {:#?}", range);
                result += range.volume();
            },
            Rule::Reject => (), // should not happen
            Rule::GoTo(target) => {
                let id = workflows.to_id(&target);
                let curr_rules = &workflows.seq[id];
                ranges.append(&mut apply_range(&range, curr_rules));
            },
            Rule::Greater(cmd, value, decision) => {

                // cmd > value => cmd in [value, high)
                // cmd > value => [low, value) (value, high]
                //let (low, high) = range.split(cmd, value - 1);
                let high = range.split_to_gt(cmd, value);
                //dbg!(&low, &high, value, &decision);
                match decision {
                    Decision::Accept => ranges.push((high, Rule::Accept)),
                    Decision::GoTo(target) => {
                        ranges.push((high, Rule::GoTo(target)));
                    }
                    // Decision::Reject => ranges.append((low, Rule::Reject)),
                    _other => (), // ignore other rules
                }
            },
            Rule::Less(cmd, value, decision) => {
                // cmd < value => cmd in [low, value)
                //let (low, high) = range.split(cmd, value);
                let low = range.split_to_lt(cmd, value);
                match decision {
                    Decision::Accept => ranges.push((low, Rule::Accept)),
                    Decision::GoTo(target) => {
                        ranges.push((low, Rule::GoTo(target)));
                    }
                    _other => (), // ignore other rules
                }
            },
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
    fn test_range_split() {
        let range = PartRange::new();
        let (l, h) = range.split(Cmd::M, 1000);
        dbg!(&l, &h);
        assert_eq!(l.volume(), (4000 as u64).pow(3) * 1000);
        assert_eq!(h.volume(), (4000 as u64).pow(3) * 3000);

        //assert_eq!(range.volume(), l.volume() + h.volume());
    }

    // #[test]
    // fn test_split_gt() {
    //     let cmd = Cmd::A;
    //     let value = 123;
    //     let gt = Rule::Greater(cmd, value, Decision::Accept);

    //     let range = PartRange::new();
    //     let (low, high) = range.split(cmd, value);
    
    //     assert_eq!(high.contains(cmd, value), true);
    //     assert_eq!(high.contains(cmd, value - 1), false);
    //     assert_eq!(low.contains(cmd, value), false);
    //     assert_eq!(low.contains(cmd, value - 1), true);
    // }

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

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
        let expected_result = 167409079868000;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
