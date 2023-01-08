use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Nums = HashSet<usize>;
type Cache = HashMap<usize, usize>;

fn main() {
    let mut nums = include_str!("input.txt")
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Nums>();

    let mine = nums.iter().max().unwrap() + 3;
    let mut cache = Cache::new();

    nums.insert(0);
    nums.insert(mine);

    println!("{}", get_diff_score(&nums));
    println!("{}", count_paths_to(&nums, &mut cache, 0, mine));
}

fn get_diff_score(nums: &Nums) -> usize {
    let diffs = nums
        .iter()
        .sorted()
        .tuple_windows::<(&usize, &usize)>()
        .map(|(a, b)| b - a)
        .counts();

    diffs.get(&1).unwrap() * diffs.get(&3).unwrap()
}

fn count_paths_to(nums: &Nums, cache: &mut Cache, curr: usize, end: usize) -> usize {
    if curr == end {
        1
    } else {
        (1..4)
            .into_iter()
            .filter(|d| nums.contains(&(curr + d)))
            .map(|d| {
                let n = curr + d;
                let cached_count = cache.get(&n);

                if let Some(count) = cached_count {
                    *count
                } else {
                    let count = count_paths_to(nums, cache, curr + d, end);
                    cache.insert(n, count);
                    count
                }
            })
            .sum()
    }
}
