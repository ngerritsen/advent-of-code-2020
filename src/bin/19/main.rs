use itertools::Itertools;
use std::collections::HashMap;

enum Rule {
    Val(char),
    Ref(Vec<Vec<u32>>),
}
type Rules = HashMap<u32, Rule>;

fn main() {
    let (rule_input, message_input) = split_input(include_str!("input.txt"));
    let messages = parse_messages(message_input);
    let rules = parse_rules(rule_input);
    let fixed_rules = parse_rules(&fix_input(rule_input));

    println!("{}", count_matches(&rules, &messages));
    println!("{}", count_matches(&fixed_rules, &messages));
}

fn count_matches(rules: &Rules, messages: &[String]) -> usize {
    messages.iter().filter(|m| matches(rules, m)).count()
}

fn matches(rules: &Rules, msg: &String) -> bool {
    get_matches(rules, &0, msg).iter().any(|r| r.is_empty())
}

fn get_matches(rules: &Rules, rule: &u32, msg: &String) -> Vec<String> {
    match rules.get(rule).unwrap() {
        Rule::Val(num) => {
            if !msg.is_empty() && *num == msg.chars().next().unwrap() {
                vec![msg.chars().skip(1).join("")]
            } else {
                vec![]
            }
        }
        Rule::Ref(options) => options
            .iter()
            .flat_map(|o| {
                o.iter().fold(vec![msg.clone()], |acc, part| {
                    acc.iter().fold(vec![], |paths, path| {
                        [get_matches(rules, part, path), paths].concat()
                    })
                })
            })
            .collect(),
    }
}

fn fix_input(input: &str) -> String {
    input
        .lines()
        .map(|l| {
            if l.starts_with("8:") {
                "8: 42 | 42 8"
            } else if l.starts_with("11:") {
                "11: 42 31 | 42 11 31"
            } else {
                l
            }
        })
        .join("\n")
}

fn split_input(input: &str) -> (&str, &str) {
    input.splitn(2, "\n\n").collect_tuple().unwrap()
}

fn parse_rules(input: &str) -> Rules {
    input
        .lines()
        .map(|l| {
            let (key, value) = l.splitn(2, ": ").collect_tuple().unwrap();
            (
                key.parse().unwrap(),
                if value.starts_with('"') {
                    Rule::Val(value.chars().nth(1).unwrap())
                } else {
                    Rule::Ref(
                        value
                            .split(" | ")
                            .map(|b| b.split(' ').map(|n| n.parse().unwrap()).collect())
                            .collect(),
                    )
                },
            )
        })
        .collect()
}

fn parse_messages(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}
