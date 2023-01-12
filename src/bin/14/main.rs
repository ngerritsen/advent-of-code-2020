use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("{}", run_program(include_str!("input.txt"), false));
    println!("{}", run_program(include_str!("input.txt"), true));
}

fn run_program(input: &str, mask_mem: bool) -> u64 {
    let mut mem: HashMap<u64, u64> = HashMap::new();
    let mut mask = "";

    input.lines().for_each(|l| {
        let (left, right) = l.splitn(2, " = ").collect_tuple::<(&str, &str)>().unwrap();
        if left == "mask" {
            mask = right;
        } else {
            let addr = parse_addr(left);
            let num = right.parse::<u64>().unwrap();

            if mask_mem {
                for a in mask_addr(addr, mask).iter() {
                    mem.insert(*a, num as u64);
                }
            } else {
                mem.insert(addr, mask_num(num, mask));
            }
        }
    });

    mem.values().sum()
}

fn parse_addr(key: &str) -> u64 {
    key.trim_start_matches("mem[")
        .trim_end_matches(']')
        .parse()
        .unwrap()
}

fn mask_addr(addr: u64, mask: &str) -> Vec<u64> {
    let addr = get_bin_str(addr, mask.len());
    mask_addr_part(addr, mask, 0, ' ')
        .iter()
        .map(|s| u64::from_str_radix(s, 2).unwrap())
        .collect()
}

fn mask_addr_part(mut addr: String, mask: &str, pos: usize, x: char) -> Vec<String> {
    for (i, c) in mask.chars().enumerate().skip(pos) {
        if (i != pos || x == ' ') && c == 'X' {
            return [
                mask_addr_part(addr.clone(), mask, i, '0'),
                mask_addr_part(addr.clone(), mask, i, '1'),
            ]
            .concat();
        } else if c == 'X' {
            addr.replace_range(i..i + 1, x.to_string().as_str());
        } else if c == '1' {
            addr.replace_range(i..i + 1, c.to_string().as_str())
        }
    }

    vec![addr]
}

fn mask_num(num: u64, mask: &str) -> u64 {
    let mask_chars = mask.chars().collect_vec();
    let masked = get_bin_str(num, mask.len())
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if mask_chars[i] == 'X' {
                c
            } else {
                mask_chars[i]
            }
        })
        .join("");

    u64::from_str_radix(masked.as_str(), 2).unwrap()
}

fn get_bin_str(num: u64, len: usize) -> String {
    let bin_str = format!("{num:b}");
    "0".repeat(len - bin_str.len()) + bin_str.as_str()
}
