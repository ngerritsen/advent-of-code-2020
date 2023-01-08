use itertools::Itertools;
use std::collections::HashMap;

type Bags = HashMap<String, HashMap<String, usize>>;
const MY_BAG: &str = "shiny gold";

fn main() {
    let bags = parse_bags(include_str!("input.txt"));

    println!("{}", count_bags_containing_mine(&bags));
    println!("{}", count_bags_in(&bags, &MY_BAG.to_string()));
}

fn count_bags_in(bags: &Bags, bag: &String) -> usize {
    bags.get(bag)
        .unwrap()
        .iter()
        .map(|(k, v)| (count_bags_in(bags, k) + 1) * v)
        .sum()
}

fn count_bags_containing_mine(bags: &Bags) -> usize {
    bags.keys()
        .filter(|b| *b != MY_BAG && has_my_bag(bags, b))
        .count()
}

fn has_my_bag(bags: &Bags, bag: &String) -> bool {
    if bag == MY_BAG {
        true
    } else {
        let children = bags.get(bag).unwrap();
        children.clone().into_keys().any(|b| has_my_bag(bags, &b))
    }
}

fn parse_bags(input: &str) -> Bags {
    input
        .lines()
        .map(|l| {
            let parts = l.split(" contain ").collect_vec();
            let key = parts[0].split(' ').take(2).join(" ");
            let value = parts[1]
                .split(", ")
                .filter(|s| !s.starts_with('n'))
                .map(|s| {
                    (
                        s.split(' ').skip(1).take(2).join(" "),
                        s.split(' ').next().unwrap().parse::<usize>().unwrap(),
                    )
                })
                .collect::<HashMap<_, _>>();

            (key, value)
        })
        .collect::<HashMap<_, _>>()
}
