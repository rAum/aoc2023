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

#[derive(Debug)]
struct MachinePart {
    data: [i64; 4],
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
    rules: HashMap<String, usize>,
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

impl Workflows {
    fn new(input: &str) -> Self {
        let mut rules = HashMap::new();
        let mut seq = Vec::new();

        let re = Regex::new(r"(?P<name>\w+)\{(?P<cmds>.+)\}").unwrap();
        let rule_re =
            Regex::new(r"(?P<cmd>[xmas])(?P<op>[<>])(?P<value>\d+):(?P<target>.+)").unwrap();

        input.lines().for_each(|line| {
            println!("Checking {} line!", line);
            let caps = re.captures(line).unwrap();
            let name = &caps["name"];
            let ruleset: Vec<Rule> = caps["cmds"]
                .split(",")
                .map(|r| {
                    let cap = rule_re.captures(r);
                    dbg!(&cap);
                    let rule = match cap {
                        None => {
                            println!("not matching! {}", r);
                            match r {
                                "A" => Rule::Accept,
                                "R" => Rule::Reject,
                                other => Rule::GoTo(other.to_string()),
                            }
                        }
                        Some(x) => {
                            println!("matching {:#?}", x);
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
                    println!("{:#?}", r);
                    rule
                })
                .collect();
            println!("{:#?} {:#?}", name, ruleset);

            rules.insert(name.to_string(), seq.len());
            seq.push(ruleset);
            println!("Checking {} line DONE", line);
        });

        Workflows { seq, rules }
    }
}

fn parse_machine_parts(input: &str) -> Vec<MachinePart> {
    let reg = Regex::new(r"(?P<cmd>[xmas])=(?P<value>\d+),?").unwrap();

    let parts: Vec<MachinePart> = input
        .lines()
        .map(|line| {
            let mut data = [0; 4];
            for mat in reg.find_iter(line) {
                let captures = reg.captures(mat.as_str()).unwrap();
                let cmd = to_cat_index(captures.name("cmd").unwrap().as_str());
                data[cmd] = captures.name("value").unwrap().as_str().parse().unwrap();
            }
            MachinePart { data }
        })
        .collect();
    parts
}

fn solution(input: &str) -> i64 {
    let mut in_rules_parts = input.split("\n\n");
    let rules = in_rules_parts.next().unwrap();
    let parts = in_rules_parts.next().unwrap();

    let parts = parse_machine_parts(parts);
    let rules = Workflows::new(rules);

    dbg!(rules);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let expected_result = 19114;

        let result = solution(test_input);

        assert_eq!(result, expected_result);
    }
}
