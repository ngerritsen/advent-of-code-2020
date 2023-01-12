use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::ops::Range;

type Rule = Vec<Range<usize>>;
type Rules<'a> = HashMap<&'a str, Rule>;
type Ticket = Vec<usize>;

fn main() {
    let (rules, my_ticket, nearby_tickets) = parse_data(include_str!("input.txt"));

    println!("{}", get_error_rate(&rules, &nearby_tickets));
    println!(
        "{}",
        get_departure_product(&rules, my_ticket, &nearby_tickets)
    );
}

fn get_error_rate(rules: &Rules, nearby_tickets: &[Ticket]) -> usize {
    nearby_tickets
        .iter()
        .flatten()
        .filter(|v| !is_valid(rules, v))
        .sum()
}

fn get_departure_product(rules: &Rules, ticket: Ticket, nearby_tickets: &[Ticket]) -> usize {
    let valid_tickets = get_valid_tickets(rules, nearby_tickets);
    let field_mapping = resolve_field_mapping(rules, &valid_tickets);

    field_mapping
        .iter()
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, v)| ticket[*v])
        .product()
}

fn get_valid_tickets(rules: &Rules, tickets: &[Ticket]) -> Vec<Ticket> {
    tickets
        .iter()
        .filter(|t| t.iter().all(|v| is_valid(rules, v)))
        .cloned()
        .collect()
}

fn resolve_field_mapping<'a>(rules: &'a Rules, tickets: &[Ticket]) -> HashMap<&'a str, usize> {
    let index_count = tickets.iter().next().unwrap().len();
    let mut settled = HashSet::new();
    let mut mapping = HashMap::new();
    let mut indices = get_matching_indices(rules, tickets);

    while settled.len() < index_count {
        let mut next_indices = indices.clone();
        let (found_key, found_list) = indices
            .iter()
            .find(|(_, i)| i.len() == 1 && !settled.contains(i.iter().next().unwrap()))
            .unwrap();

        let found = *found_list.iter().next().unwrap();

        indices
            .iter()
            .filter(|(_, i)| i.len() > 1)
            .for_each(|(k, v)| {
                next_indices.insert(*k, v.iter().filter(|v| **v != found).copied().collect());
            });

        settled.insert(found);
        mapping.insert(*found_key, found);

        indices = next_indices;
    }

    mapping
}

fn get_matching_indices<'a>(rules: &'a Rules, tickets: &[Ticket]) -> HashMap<&'a str, Vec<usize>> {
    let index_count = tickets.iter().next().unwrap().len();
    let mut indices: HashMap<_, _> = rules.keys().map(|k| (*k, Vec::new())).collect();

    (0..index_count).for_each(|i| {
        let mut match_counts: HashMap<&str, usize> = rules.keys().map(|k| (*k, 0)).collect();

        tickets.iter().for_each(|t| {
            rules.iter().for_each(|(k, r)| {
                if matches(r, &t[i]) {
                    match_counts.insert(*k, match_counts.get(k).unwrap() + 1);
                }
            });
        });

        match_counts.iter().for_each(|(k, v)| {
            if *v == tickets.len() {
                indices.get_mut(k).unwrap().push(i);
            }
        });
    });

    indices
}

fn is_valid(rules: &Rules, val: &usize) -> bool {
    rules.values().any(|rule| matches(rule, val))
}

fn matches(rule: &Rule, val: &usize) -> bool {
    rule.iter().any(|r| r.contains(val))
}

fn parse_data(input: &str) -> (Rules, Ticket, Vec<Ticket>) {
    let parts = input.split("\n\n").collect_vec();

    let rules = parts[0]
        .lines()
        .map(|l| l.splitn(2, ": ").collect_tuple::<(&str, &str)>().unwrap())
        .map(|(k, v)| (k, v.splitn(2, " or ").map(parse_range).collect()))
        .collect();

    let ticket = parse_ticket(parts[1].lines().last().unwrap());
    let nearby_tickets = parts[2].lines().skip(1).map(parse_ticket).collect();

    (rules, ticket, nearby_tickets)
}

fn parse_range(raw: &str) -> Range<usize> {
    let (from, to) = raw
        .splitn(2, '-')
        .map(|s| s.parse().unwrap())
        .collect_tuple::<(usize, usize)>()
        .unwrap();

    from..(to + 1)
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(',').map(|c| c.parse().unwrap()).collect()
}
