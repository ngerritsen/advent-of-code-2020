extern crate core;

use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type Passport<'a> = HashMap<&'a str, &'a str>;

fn main() {
    let req = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let passports = parse_passports(include_str!("input.txt"));

    println!("{}", count_valid_v1(&passports, &req));
    println!("{}", count_valid_v2(&passports, &req));
}

fn count_valid_v1(passports: &[Passport], req: &[&str]) -> usize {
    passports
        .iter()
        .filter(|p| req.iter().all(|k| p.contains_key(k)))
        .count()
}

fn count_valid_v2(passports: &[Passport], req: &[&str]) -> usize {
    let eye_colors = HashSet::from(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]);
    let hcl_re = Regex::new(r"^#[a-f|\d]{6}$").unwrap();
    let pid_re = Regex::new(r"^\d{9}$").unwrap();

    passports
        .iter()
        .filter(|p| {
            req.iter().all(|k| p.contains_key(k))
                && p.iter().all(|(k, v)| match *k {
                    "byr" => between(v, 1920, 2002),
                    "iyr" => between(v, 2010, 2020),
                    "eyr" => between(v, 2020, 2030),
                    "hgt" => {
                        if v.ends_with("in") {
                            between(v.trim_end_matches("in"), 59, 76)
                        } else if v.ends_with("cm") {
                            between(v.trim_end_matches("cm"), 150, 193)
                        } else {
                            false
                        }
                    }
                    "hcl" => hcl_re.is_match(v),
                    "ecl" => eye_colors.contains(v),
                    "pid" => pid_re.is_match(v),
                    "cid" => true,
                    _ => false,
                })
        })
        .count()
}

fn between(v: &str, l: i32, r: i32) -> bool {
    (l..r + 1).contains(&v.parse::<i32>().unwrap())
}

fn parse_passports(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|p| {
            p.split('\n')
                .flat_map(|s| s.split(' '))
                .map(|f| f.split(':').collect_tuple().unwrap())
                .collect::<Passport>()
        })
        .collect()
}
