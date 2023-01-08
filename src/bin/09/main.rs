extern crate core;

use itertools::{max, min, Itertools};

fn main() {
    let nums = include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    let invalid_num = get_invalid_num(&nums, 25);

    println!("{}", invalid_num);
    println!("{}", get_weakness(&nums, invalid_num));
}

fn get_weakness(nums: &[usize], invalid_num: usize) -> usize {
    let res = (2..nums.len())
        .into_iter()
        .map(|s| {
            nums.windows(s)
                .find(|w| w.iter().sum::<usize>() == invalid_num)
        })
        .find(|r| r.is_some())
        .unwrap()
        .unwrap();

    min(res).unwrap() + max(res).unwrap()
}

fn get_invalid_num(nums: &[usize], window_size: usize) -> usize {
    *nums
        .windows(window_size + 1)
        .into_iter()
        .find(|w| {
            !w.iter()
                .take(window_size)
                .combinations(2)
                .map(|c| c.iter().copied().sum::<usize>())
                .contains(w.iter().last().unwrap())
        })
        .unwrap()
        .last()
        .unwrap()
}
